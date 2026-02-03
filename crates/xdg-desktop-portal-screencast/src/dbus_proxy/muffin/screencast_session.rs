#[zbus::proxy(
    interface = "org.cinnamon.Muffin.ScreenCast.Session",
    default_service = "org.cinnamon.Muffin.ScreenCast"
)]
pub trait Session {
    fn record_monitor(
        &self,
        connector: &str,
        properties: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
    fn record_window(
        &self,
        properties: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
    fn start(&self) -> zbus::Result<()>;
    fn stop(&self) -> zbus::Result<()>;

    #[zbus(signal)]
    fn closed(&self) -> zbus::Result<()>;
}
