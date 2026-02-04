fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(xdg_desktop_portal_screencast::run())
}
