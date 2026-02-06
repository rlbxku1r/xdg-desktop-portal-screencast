use super::ScreenCastSession as Session;
use crate::dbus_proxy;
use std::{collections::HashMap, str::FromStr};
use zbus::zvariant;

const SOURCE_TYPE_MONITOR: u32 = 1 << 0;
const SOURCE_TYPE_WINDOW: u32 = 1 << 1;
const _SOURCE_TYPE_VIRTUAL: u32 = 1 << 2;

const CURSOR_TYPE_HIDDEN: u32 = 1 << 0;
const _CURSOR_TYPE_EMBEDDED: u32 = 1 << 1;
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
        log::debug!("CreateSession():");
        log::debug!("\thandle: {handle}");
        log::debug!("\tsession_handle: {session_handle}");
        log::debug!("\tapp_id: {app_id}");
        log::debug!("\toptions: {options:?}");

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
        log::debug!("SelectSources():");
        log::debug!("\thandle: {handle}");
        log::debug!("\tsession_handle: {session_handle}");
        log::debug!("\tapp_id: {app_id}");
        log::debug!("\toptions: {options:?}");

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
        log::debug!("Start():");
        log::debug!("\thandle: {handle}");
        log::debug!("\tsession_handle: {session_handle}");
        log::debug!("\tapp_id: {app_id}");
        log::debug!("\tparent_window: {parent_window}");
        log::debug!("\toptions: {options:?}");

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
    screencast_proxy: dbus_proxy::muffin::ScreenCast<'a>,
    screencast_sessions: HashMap<zvariant::OwnedObjectPath, Session<'a>>,
}

impl<'a> ScreenCastInner<'a> {
    async fn new(connection: zbus::Connection) -> zbus::Result<Self> {
        let screencast_proxy = dbus_proxy::muffin::ScreenCast::new(&connection).await?;

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
        _options: HashMap<String, zvariant::OwnedValue>,
    ) -> (u32, HashMap<String, zvariant::OwnedValue>) {
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
                log::error!("create_session(): {err}");
                _ = self.emit_request_ack(&handle, 2, HashMap::new()).await;
                (1, HashMap::new())
            }
        }
    }

    async fn select_sources(
        &mut self,
        handle: zvariant::OwnedObjectPath,
        session_handle: zvariant::OwnedObjectPath,
        _app_id: String,
        _options: HashMap<String, zvariant::OwnedValue>,
    ) -> (u32, HashMap<String, zvariant::OwnedValue>) {
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
                log::error!("select_sources(): {err}");
                _ = self.emit_request_ack(&handle, 2, HashMap::new()).await;
                (1, HashMap::new())
            }
        }
    }

    async fn start(
        &self,
        handle: zvariant::OwnedObjectPath,
        session_handle: zvariant::OwnedObjectPath,
        _app_id: String,
        _parent_window: String,
        _options: HashMap<String, zvariant::OwnedValue>,
    ) -> (u32, HashMap<String, zvariant::OwnedValue>) {
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
            log::debug!("ScreenCast started on PipeWire stream ID: {pipewire_stream_id}");
            zbus::Result::Ok([("streams".into(), zvariant::OwnedValue::try_from(streams)?)].into())
        };
        match body.await {
            Ok(streams) => {
                _ = self.emit_request_ack(&handle, 0, HashMap::new()).await;
                (0, streams)
            }
            Err(err) => {
                log::error!("start(): {err}");
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
        CURSOR_TYPE_HIDDEN | CURSOR_TYPE_METADATA
    }

    fn available_source_types() -> u32 {
        SOURCE_TYPE_MONITOR | SOURCE_TYPE_WINDOW
    }

    fn version() -> u32 {
        5
    }
}
