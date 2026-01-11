#[zbus::proxy(
    interface = "org.cinnamon.Muffin.Display",
    default_service = "org.cinnamon.Muffin.Display",
    default_path = "/org/cinnamon/Muffin/Display"
)]
pub trait Display {
    fn list_windows(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;
}
