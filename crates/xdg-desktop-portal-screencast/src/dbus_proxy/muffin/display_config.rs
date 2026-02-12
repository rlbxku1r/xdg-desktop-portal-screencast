#[zbus::proxy(
    interface = "org.cinnamon.Muffin.DisplayConfig",
    default_service = "org.cinnamon.Muffin.DisplayConfig",
    default_path = "/org/cinnamon/Muffin/DisplayConfig",
    gen_blocking = false
)]
pub trait DisplayConfig {
    #[allow(clippy::type_complexity)]
    fn apply_configuration(
        &self,
        serial: u32,
        persistent: bool,
        crtcs: &[&(
            u32,
            i32,
            i32,
            i32,
            u32,
            &[u32],
            std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
        )],
        outputs: &[&(
            u32,
            std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
        )],
    ) -> zbus::Result<()>;
    #[allow(clippy::type_complexity)]
    fn apply_monitors_config(
        &self,
        serial: u32,
        logical_monitors: &[&(
            i32,
            i32,
            f64,
            u32,
            bool,
            &[&(
                &str,
                &str,
                std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
            )],
        )],
        properties: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;
    fn change_backlight(&self, serial: u32, output: u32, value: i32) -> zbus::Result<i32>;
    fn get_crtc_gamma(
        &self,
        serial: u32,
        crtc: u32,
    ) -> zbus::Result<(Vec<u16>, Vec<u16>, Vec<u16>)>;
    #[allow(clippy::type_complexity)]
    fn get_current_state(
        &self,
    ) -> zbus::Result<(
        u32,
        Vec<(
            (String, String, String, String),
            Vec<(
                String,
                i32,
                i32,
                f64,
                f64,
                Vec<f64>,
                std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
            )>,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        )>,
        Vec<(
            i32,
            i32,
            f64,
            u32,
            bool,
            Vec<(String, String, String, String)>,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        )>,
        std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
    )>;
    #[allow(clippy::type_complexity)]
    fn get_resources(
        &self,
    ) -> zbus::Result<(
        u32,
        Vec<(
            u32,
            i64,
            i32,
            i32,
            i32,
            i32,
            i32,
            u32,
            Vec<u32>,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        )>,
        Vec<(
            u32,
            i64,
            i32,
            Vec<u32>,
            String,
            Vec<u32>,
            Vec<u32>,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        )>,
        Vec<(u32, i64, u32, u32, f64, u32)>,
        i32,
        i32,
    )>;
    fn set_crtc_gamma(
        &self,
        serial: u32,
        crtc: u32,
        red: &[u16],
        green: &[u16],
        blue: &[u16],
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    fn monitors_changed(&self) -> zbus::Result<()>;

    #[zbus(property)]
    fn power_save_mode(&self) -> zbus::Result<i32>;
    #[zbus(property)]
    fn set_power_save_mode(&self, value: i32) -> zbus::Result<()>;
}
