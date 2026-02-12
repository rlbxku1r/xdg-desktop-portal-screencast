#[zbus::proxy(
    interface = "org.freedesktop.portal.Session",
    default_service = "org.freedesktop.portal.Desktop",
    gen_blocking = false
)]
pub trait Session {
    fn close(&self) -> zbus::Result<()>;

    #[zbus(signal)]
    fn closed(
        &self,
        details: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    #[zbus(property, name = "version")]
    fn version(&self) -> zbus::Result<u32>;
}
