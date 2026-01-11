use super::ScreenCastSession as Session;
use crate::muffin_proxy;
use std::{collections::HashMap, str::FromStr};
use zbus::zvariant;

const SOURCE_TYPE_MONITOR: u32 = 1 << 0;
const SOURCE_TYPE_WINDOW: u32 = 1 << 1;
const SOURCE_TYPE_VIRTUAL: u32 = 1 << 2;

const CURSOR_TYPE_HIDDEN: u32 = 1 << 0;
const CURSOR_TYPE_EMBEDDED: u32 = 1 << 1;
const CURSOR_TYPE_METADATA: u32 = 1 << 2;

#[derive(Clone)]
pub struct ScreenCast {
    inner: std::sync::Arc<tokio::sync::Mutex<ScreenCastInner<'static>>>,
}

impl ScreenCast {
    pub async fn new(connection: zbus::Connection) -> zbus::Result<Self> {
        let inner = std::sync::Arc::new(tokio::sync::Mutex::new(
            ScreenCastInner::new(connection).await?,
        ));

        Ok(Self { inner })
    }

    pub async fn on_app_closed(&self, app_id: &str) {
        self.inner.lock().await.on_app_closed(app_id).await
    }
}

#[zbus::interface(name = "org.freedesktop.impl.portal.ScreenCast")]
impl ScreenCast {
    async fn create_session(
        &self,
        handle: zvariant::OwnedObjectPath,
        session_handle: zvariant::OwnedObjectPath,
        app_id: String,
        options: HashMap<String, zvariant::OwnedValue>,
    ) -> (u32, HashMap<String, zvariant::OwnedValue>) {
        self.inner
            .lock()
            .await
            .create_session(handle, session_handle, app_id, options)
            .await
    }

    async fn select_sources(
        &self,
        handle: zvariant::OwnedObjectPath,
        session_handle: zvariant::OwnedObjectPath,
        app_id: String,
        options: HashMap<String, zvariant::OwnedValue>,
    ) -> (u32, HashMap<String, zvariant::OwnedValue>) {
        self.inner
            .lock()
            .await
            .select_sources(handle, session_handle, app_id, options)
            .await
    }

    async fn start(
        &self,
        handle: zvariant::OwnedObjectPath,
        session_handle: zvariant::OwnedObjectPath,
        app_id: String,
        parent_window: String,
        options: HashMap<String, zvariant::OwnedValue>,
    ) -> (u32, HashMap<String, zvariant::OwnedValue>) {
        self.inner
            .lock()
            .await
            .start(handle, session_handle, app_id, parent_window, options)
            .await
    }

    #[zbus(property)]
    fn available_cursor_modes(&self) -> u32 {
        ScreenCastInner::available_cursor_modes()
    }

    #[zbus(property)]
    fn available_source_types(&self) -> u32 {
        ScreenCastInner::available_source_types()
    }

    #[zbus(property, name = "version")]
    fn version(&self) -> u32 {
        ScreenCastInner::version()
    }
}

struct ScreenCastInner<'a> {
    connection: zbus::Connection,
    screencast_proxy: muffin_proxy::ScreenCast<'a>,
    screencast_sessions: HashMap<zvariant::OwnedObjectPath, Session<'a>>,
}

impl<'a> ScreenCastInner<'a> {
    async fn new(connection: zbus::Connection) -> zbus::Result<Self> {
        let screencast_proxy = muffin_proxy::ScreenCast::new(&connection).await?;

        Ok(Self {
            connection,
            screencast_proxy,
            screencast_sessions: HashMap::new(),
        })
    }

    async fn on_app_closed(&mut self, app_id: &str) {
        let iter = self
            .screencast_sessions
            .extract_if(|_, x| x.get_app_id() == app_id);
        for (_, session) in iter {
            session.close().await;
        }
    }

    async fn create_session(
        &mut self,
        handle: zvariant::OwnedObjectPath,
        session_handle: zvariant::OwnedObjectPath,
        app_id: String,
        options: HashMap<String, zvariant::OwnedValue>,
    ) -> (u32, HashMap<String, zvariant::OwnedValue>) {
        #[cfg(debug_assertions)]
        {
            eprintln!("create_session():");
            eprintln!("\thandle: {handle}");
            eprintln!("\tsession_handle: {session_handle}");
            eprintln!("\tapp_id: {app_id}");
            eprintln!("\toptions: {options:?}");
            /*
               handle: /org/freedesktop/portal/desktop/request/1_148/obs1
               session_handle: /org/freedesktop/portal/desktop/session/1_148/obs1
               app_id: com.obsproject.Studio
               options: {}
            */
        }
        let body = async {
            let connection = self.connection.clone();
            let session_object_path = self.screencast_proxy.create_session(HashMap::new()).await?;
            let session =
                Session::new(connection, app_id, &session_handle, &session_object_path).await?;
            self.screencast_sessions.insert(session_handle, session);
            zbus::Result::Ok(())
        };
        match body.await {
            Ok(_) => {
                _ = self.emit_request_ack(&handle, 0, HashMap::new()).await;
                (0, HashMap::new())
            }
            Err(err) => {
                eprintln!("create_session error: {err}");
                _ = self.emit_request_ack(&handle, 2, HashMap::new()).await;
                (1, HashMap::new())
            }
        }
    }

    async fn select_sources(
        &mut self,
        handle: zvariant::OwnedObjectPath,
        session_handle: zvariant::OwnedObjectPath,
        app_id: String,
        options: HashMap<String, zvariant::OwnedValue>,
    ) -> (u32, HashMap<String, zvariant::OwnedValue>) {
        #[cfg(debug_assertions)]
        {
            eprintln!("select_sources():");
            eprintln!("\thandle: {handle}");
            eprintln!("\tsession_handle: {session_handle}");
            eprintln!("\tapp_id: {app_id}");
            eprintln!("\toptions: {options:?}");
            /*
               handle: /org/freedesktop/portal/desktop/request/1_148/obs2
               session_handle: /org/freedesktop/portal/desktop/session/1_148/obs1
               app_id: com.obsproject.Studio
               options: {"multiple": Bool(false), "cursor_mode": U32(4), "types": U32(3), "persist_mode": U32(2)}
            */
        }
        let body = async {
            self.screencast_sessions
                .get_mut(&session_handle)
                .ok_or_else(|| {
                    zbus::Error::Failure(format!("session for '{session_handle}' not found"))
                })?
                .select_sources()
                .await
        };
        match body.await {
            Ok(_) => {
                _ = self.emit_request_ack(&handle, 0, HashMap::new()).await;
                (0, HashMap::new())
            }
            Err(err) => {
                eprintln!("select_sources error: {err}");
                _ = self.emit_request_ack(&handle, 2, HashMap::new()).await;
                (1, HashMap::new())
            }
        }
    }

    async fn start(
        &self,
        handle: zvariant::OwnedObjectPath,
        session_handle: zvariant::OwnedObjectPath,
        app_id: String,
        parent_window: String,
        options: HashMap<String, zvariant::OwnedValue>,
    ) -> (u32, HashMap<String, zvariant::OwnedValue>) {
        #[cfg(debug_assertions)]
        {
            eprintln!("start():");
            eprintln!("\thandle: {handle}");
            eprintln!("\tsession_handle: {session_handle}");
            eprintln!("\tapp_id: {app_id}");
            eprintln!("\tparent_window: {parent_window}");
            eprintln!("\toptions: {options:?}");
            /*
               handle: /org/freedesktop/portal/desktop/request/1_148/obs3
               session_handle: /org/freedesktop/portal/desktop/session/1_148/obs1
               app_id: com.obsproject.Studio
               parent_window:
               options: {}
            */
        }
        let body = async {
            let session = self
                .screencast_sessions
                .get(&session_handle)
                .ok_or_else(|| {
                    zbus::Error::Failure(format!("session for '{session_handle}' not found"))
                })?;
            let pipewire_stream_id = session.start().await?;

            let mut streams = zvariant::Array::new(&zvariant::Signature::from_str("(ua{sv})")?);
            let stream_info = zvariant::StructureBuilder::new()
                .add_field(pipewire_stream_id)
                .add_field(HashMap::<&str, zvariant::Value>::new())
                .build()?;
            streams.append(stream_info.into())?;
            println!("ScreenCast started on PipeWire stream ID: {pipewire_stream_id}");
            zbus::Result::Ok(HashMap::from([(
                "streams".into(),
                zvariant::OwnedValue::try_from(streams)?,
            )]))
        };
        match body.await {
            Ok(streams) => {
                _ = self.emit_request_ack(&handle, 0, HashMap::new()).await;
                (0, streams)
            }
            Err(err) => {
                eprintln!("start error: {err}");
                _ = self.emit_request_ack(&handle, 2, HashMap::new()).await;
                (1, HashMap::new())
            }
        }
    }

    async fn emit_request_ack(
        &self,
        request_object: &zvariant::ObjectPath<'_>,
        response: u32,
        results: HashMap<String, zvariant::OwnedValue>,
    ) -> zbus::Result<()> {
        self.connection
            .emit_signal(
                Some("org.freedesktop.portal.Desktop"),
                request_object,
                "org.freedesktop.portal.Request",
                "Response",
                &(response, results),
            )
            .await
    }

    fn available_cursor_modes() -> u32 {
        CURSOR_TYPE_HIDDEN | CURSOR_TYPE_EMBEDDED | CURSOR_TYPE_METADATA
    }

    fn available_source_types() -> u32 {
        SOURCE_TYPE_MONITOR | SOURCE_TYPE_WINDOW | SOURCE_TYPE_VIRTUAL
    }

    fn version() -> u32 {
        5
    }
}
