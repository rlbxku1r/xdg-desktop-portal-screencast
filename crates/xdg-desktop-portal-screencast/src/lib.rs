mod dbus_proxy;
mod portal_impl;
mod running_app_watcher;
mod sigint_handler;

use std::sync::atomic::Ordering;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let connection = zbus::connection::Builder::session()?
        .name("org.freedesktop.impl.portal.desktop.screencast")?
        .build()
        .await?;
    let screencast_ctx = portal_impl::ScreenCast::new(connection.clone()).await?;
    connection
        .object_server()
        .at("/org/freedesktop/portal/desktop", screencast_ctx.clone())
        .await?;

    running_app_watcher::setup(&connection, screencast_ctx).await?;

    let sigint_caught = sigint_handler::setup();

    while !sigint_caught.load(Ordering::Relaxed) {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    Ok(())
}
