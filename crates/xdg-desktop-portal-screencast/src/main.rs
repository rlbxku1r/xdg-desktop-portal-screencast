fn main() -> Result<(), Box<dyn std::error::Error>> {
    let format_target = std::env::var("RUST_LOG_FORMAT_TARGET").is_ok_and(|x| x == "1");
    env_logger::builder().format_target(format_target).init();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(xdg_desktop_portal_screencast::run())
}
