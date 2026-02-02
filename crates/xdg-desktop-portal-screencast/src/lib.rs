mod cinnamon_proxy;
mod muffin_proxy;
mod portal_impl;
mod sigint;
mod xdg_desktop_portal_proxy;

use futures_util::StreamExt;
use std::{
    collections::HashSet,
    sync::atomic::{AtomicBool, Ordering},
};

static SIGINT_RECEIVED: AtomicBool = AtomicBool::new(false);

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

    setup_running_apps_watcher(&connection, screencast_ctx).await?;

    sigint::set_callback(|| {
        SIGINT_RECEIVED.store(true, Ordering::Relaxed);
    });

    while !SIGINT_RECEIVED.load(Ordering::Relaxed) {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    Ok(())
}

async fn setup_running_apps_watcher(
    connection: &zbus::Connection,
    screencast_ctx: portal_impl::ScreenCast,
) -> zbus::Result<()> {
    let portal_handlers = cinnamon_proxy::PortalHandlers::new(connection).await?;
    let mut stream = portal_handlers.receive_running_apps_changed().await?;

    tokio::spawn(async move {
        let mut last_apps = HashSet::new();
        while stream.next().await.is_some() {
            let result = portal_handlers.get_app_states().await;
            match result {
                Ok(apps) => {
                    let apps: HashSet<_> = apps
                        .keys()
                        .map(|x| x.strip_suffix(".desktop").unwrap_or(x).to_owned())
                        .filter(|x| !x.is_empty())
                        .collect();
                    for app_id in last_apps.difference(&apps) {
                        screencast_ctx.on_app_closed(app_id).await;
                    }
                    last_apps = apps;
                }
                Err(err) => eprintln!("org.cinnamon.PortalHandlers - GetAppStates(): {err}"),
            }
        }
        panic!(
            "org.cinnamon.PortalHandlers - RunningAppsChanged(): Signal stream has been terminated"
        );
    });

    Ok(())
}
