use crate::{dbus_proxy, portal_impl};
use futures_util::StreamExt;
use std::collections::HashSet;

pub async fn setup(
    connection: &zbus::Connection,
    screencast_ctx: portal_impl::ScreenCast,
) -> zbus::Result<()> {
    let portal_handlers = dbus_proxy::cinnamon::PortalHandlers::new(connection).await?;
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
                Err(err) => log::error!("org.cinnamon.PortalHandlers - GetAppStates(): {err}"),
            }
        }
        // Something went wrong in the compositor?
        panic!(
            "org.cinnamon.PortalHandlers - RunningAppsChanged(): The signal stream ended unexpectedly"
        );
    });

    Ok(())
}
