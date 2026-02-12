use crate::dbus_proxy;
use futures_util::StreamExt;

pub struct ScreenCastStream<'a> {
    screencast_stream_proxy: dbus_proxy::muffin::ScreenCastStream<'a>,
}

impl<'a> ScreenCastStream<'a> {
    pub async fn new(
        screencast_stream_proxy: dbus_proxy::muffin::ScreenCastStream<'a>,
    ) -> zbus::Result<Self> {
        Ok(Self {
            screencast_stream_proxy,
        })
    }

    pub async fn wait_for_pipewire_stream(&self) -> zbus::Result<u32> {
        let mut stream = self
            .screencast_stream_proxy
            .receive_pipewire_stream_added()
            .await?;
        let get_signal = async {
            stream.next().await.ok_or_else(|| {
                zbus::Error::Failure(
                    "The stream was closed before the 'PipeWireStreamAdded' is signaled".into(),
                )
            })
        };
        let timeout = tokio::time::sleep(std::time::Duration::from_secs(1));
        tokio::select! {
            signal = get_signal => {
                let pipewire_stream_id = signal?.message().body().deserialize::<u32>()?;
                Ok(pipewire_stream_id)
            }
            _ = timeout => {
                Err(zbus::Error::Failure("The stream was timed out before the 'PipeWireStreamAdded' is signaled".into()))
            }
        }
    }
}
