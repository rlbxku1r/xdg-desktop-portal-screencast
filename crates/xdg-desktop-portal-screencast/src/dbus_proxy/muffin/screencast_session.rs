use super::screencast_stream::*;

#[zbus::proxy(
    interface = "org.cinnamon.Muffin.ScreenCast.Session",
    default_service = "org.cinnamon.Muffin.ScreenCast",
    gen_blocking = false
)]
pub trait Session {
    #[zbus(object = "Stream")]
    fn record_monitor(
        &self,
        connector: &str,
        properties: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    );
    #[zbus(object = "Stream")]
    fn record_window(
        &self,
        properties: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    );
    fn start(&self) -> zbus::Result<()>;
    fn stop(&self) -> zbus::Result<()>;

    #[zbus(signal)]
    fn closed(&self) -> zbus::Result<()>;
}
