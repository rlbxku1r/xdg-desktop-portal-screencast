#[zbus::proxy(
    interface = "org.cinnamon.Muffin.ScreenCast",
    default_service = "org.cinnamon.Muffin.ScreenCast",
    default_path = "/org/cinnamon/Muffin/ScreenCast"
)]
pub trait ScreenCast {
    fn create_session(
        &self,
        properties: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    #[zbus(property)]
    fn version(&self) -> zbus::Result<i32>;
}
