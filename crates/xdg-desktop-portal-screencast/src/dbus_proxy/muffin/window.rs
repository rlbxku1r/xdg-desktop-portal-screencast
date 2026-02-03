#[zbus::proxy(
    interface = "org.cinnamon.Muffin.Window",
    default_service = "org.cinnamon.Muffin.Window",
    default_path = "/org/cinnamon/Muffin/Window"
)]
pub trait Window {
    fn list_windows(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;
}
