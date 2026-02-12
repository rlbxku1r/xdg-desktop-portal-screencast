use super::screencast_session::*;

#[zbus::proxy(
    interface = "org.cinnamon.Muffin.ScreenCast",
    default_service = "org.cinnamon.Muffin.ScreenCast",
    default_path = "/org/cinnamon/Muffin/ScreenCast",
    gen_blocking = false
)]
pub trait ScreenCast {
    #[zbus(object = "Session")]
    fn create_session(
        &self,
        properties: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    );

    #[zbus(property)]
    fn version(&self) -> zbus::Result<i32>;
}
