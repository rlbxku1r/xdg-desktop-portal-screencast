#[zbus::proxy(
    interface = "org.cinnamon.PortalHandlers",
    default_service = "org.Cinnamon",
    default_path = "/org/Cinnamon"
)]
pub trait PortalHandlers {
    fn get_app_states(
        &self,
    ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    #[zbus(signal)]
    fn running_apps_changed(&self) -> zbus::Result<()>;
}
