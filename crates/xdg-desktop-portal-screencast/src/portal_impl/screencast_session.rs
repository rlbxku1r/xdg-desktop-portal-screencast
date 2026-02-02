use super::ScreenCastStream;
use crate::{muffin_proxy, xdg_desktop_portal_proxy};
use libsourceselector::{SerdeJson, Source, Sources};
use std::collections::HashMap;
use zbus::zvariant;

pub struct ScreenCastSession<'a> {
    connection: zbus::Connection,
    app_id: String,
    session_proxy: xdg_desktop_portal_proxy::Session<'a>,
    screencast_session_proxy: muffin_proxy::ScreenCastSession<'a>,
    screencast_stream: Option<ScreenCastStream<'a>>,
    display_config_proxy: muffin_proxy::DisplayConfig<'a>,
    window_proxy: muffin_proxy::Window<'a>,
}

impl<'a> ScreenCastSession<'a> {
    pub async fn new<'b: 'a>(
        connection: zbus::Connection,
        app_id: String,
        session_handle: &zvariant::ObjectPath<'b>,
        session_object_path: &zvariant::ObjectPath<'b>,
    ) -> zbus::Result<Self> {
        let session_proxy =
            xdg_desktop_portal_proxy::Session::new(&connection, session_handle).await?;
        let screencast_session_proxy =
            muffin_proxy::ScreenCastSession::new(&connection, session_object_path).await?;
        let display_config_proxy = muffin_proxy::DisplayConfig::new(&connection).await?;
        let window_proxy = muffin_proxy::Window::new(&connection).await?;

        Ok(Self {
            connection,
            app_id,
            session_proxy,
            screencast_session_proxy,
            screencast_stream: None,
            display_config_proxy,
            window_proxy,
        })
    }

    pub fn get_app_id(&self) -> &String {
        &self.app_id
    }

    pub async fn select_sources(&mut self) -> zbus::Result<()> {
        let selected_source = self
            .open_source_selector()
            .await
            .map_err(|err| zbus::Error::Failure(err.to_string()))?;
        let stream_object_path = match selected_source {
            Source::Monitor { monitor_name } => {
                self.screencast_session_proxy
                    .record_monitor(&monitor_name, HashMap::new())
                    .await?
            }
            Source::Window { window_id, .. } => {
                let window_id = zvariant::Value::from(window_id);
                let properties = HashMap::from([("window-id", &window_id)]);
                self.screencast_session_proxy
                    .record_window(properties)
                    .await?
            }
        };
        self.screencast_stream =
            Some(ScreenCastStream::new(self.connection.clone(), &stream_object_path).await?);
        Ok(())
    }

    async fn get_monitor_sources(&self) -> zbus::Result<Sources> {
        let (_, _, outputs, _, _, _) = self.display_config_proxy.get_resources().await?;
        let mut monitor_sources = Vec::new();
        for output in outputs {
            let (_, _, _, _, monitor_name, _, _, _) = output;
            monitor_sources.push(Source::Monitor { monitor_name });
        }
        Ok(monitor_sources.into())
    }

    async fn get_window_sources(&self) -> zbus::Result<Sources> {
        let windows = self.window_proxy.list_windows().await?;
        let mut window_sources = Vec::new();
        for window in windows {
            let Some(Ok(window_id)) = window.get("id").map(|x| x.downcast_ref()) else {
                eprintln!("Window id isn't valid integer or unavailable, skipping...");
                continue;
            };
            let window_name = window
                .get("title")
                .and_then(|x| x.downcast_ref().ok())
                .unwrap_or_else(|| format!("<Unnamed Window: {window_id}>"));
            let icon_path = window
                .get("res_name")
                .and_then(|x| x.downcast_ref().ok())
                .and_then(get_icon_path);
            window_sources.push(Source::Window {
                window_id,
                window_name,
                icon_path,
            });
        }
        Ok(window_sources.into())
    }

    async fn open_source_selector(&self) -> Result<Source, Box<dyn std::error::Error>> {
        let mut exe = std::env::current_exe()?;
        exe.set_file_name("sourceselector-ui");
        let monitor_sources = self.get_monitor_sources().await?;
        let window_sources = self.get_window_sources().await?;
        let output = tokio::process::Command::new(exe)
            .arg(monitor_sources.to_json()?)
            .arg(window_sources.to_json()?)
            .stdout(std::process::Stdio::piped())
            .spawn()?
            .wait_with_output()
            .await?;
        let stdout = String::from_utf8(output.stdout)?;
        if stdout.is_empty() {
            return Err("sourceselector-ui did not return the answer".into());
        }
        let selected_source = Source::from_json(&stdout)?;
        Ok(selected_source)
    }

    pub async fn start(&self) -> zbus::Result<u32> {
        let screencast_stream = self.screencast_stream.as_ref().ok_or_else(|| {
            zbus::Error::Failure(
                "ScreenCastStream must be created before waiting for its PipeWire stream".into(),
            )
        })?;
        let session_start = self.screencast_session_proxy.start();
        let wait_for_pipewire_stream = screencast_stream.wait_for_pipewire_stream();
        let results = tokio::join!(session_start, wait_for_pipewire_stream);
        results.0?;
        results.1
    }

    pub async fn close(&self) {
        _ = self.screencast_session_proxy.stop().await;
        _ = self.session_proxy.close().await;
    }
}

fn get_icon_path(app_id: &str) -> Option<String> {
    static XDG_DATA_HOME: std::sync::LazyLock<String> =
        std::sync::LazyLock::new(|| match get_xdg_data_home() {
            Ok(dir) => dir,
            Err(err) => {
                eprintln!("Could not determine the $XDG_DATA_HOME directory: {err}");
                eprintln!("User *.desktop files will not be looked up");
                "".into()
            }
        });
    let search_paths: [&'static str; 3] = [&XDG_DATA_HOME, "/usr/local/share", "/usr/share"];
    for search_path in search_paths {
        if search_path.is_empty() {
            continue;
        }
        let path = format!("{search_path}/applications/{app_id}.desktop");
        let desktop_file = match ini::Ini::load_from_file(&path) {
            Ok(ini) => ini,
            Err(ini::Error::Io(err)) if err.kind() == std::io::ErrorKind::NotFound => continue,
            Err(err) => {
                eprintln!("Could not read '{path}': {err}");
                return None;
            }
        };
        let icon_name = desktop_file.section(Some("Desktop Entry"))?.get("Icon")?;
        let icon_path = freedesktop_icons::lookup(icon_name).find()?;
        return Some(format!("file://{}", icon_path.to_str()?));
    }
    None
}

fn get_xdg_data_home() -> Result<String, Box<dyn std::error::Error>> {
    match std::env::var("XDG_DATA_HOME") {
        Ok(xdg_data_home) => Ok(xdg_data_home),
        Err(std::env::VarError::NotPresent) => match std::env::var("HOME") {
            Ok(home) => Ok(format!("{home}/.local/share")),
            Err(err) => Err(format!("Failed to lookup user's home directory: {err}").into()),
        },
        Err(err) => Err(format!("Invalid $XDG_DATA_HOME variable: {err}").into()),
    }
}
