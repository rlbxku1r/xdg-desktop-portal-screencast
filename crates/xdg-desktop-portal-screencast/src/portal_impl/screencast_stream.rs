use crate::muffin_proxy;
use futures_util::StreamExt;
use zbus::zvariant;

pub struct ScreenCastStream<'a> {
    screencast_stream_proxy: muffin_proxy::ScreenCastStream<'a>,
}

impl<'a> ScreenCastStream<'a> {
    pub async fn new<'b: 'a>(
        connection: zbus::Connection,
        stream_object_path: &zvariant::ObjectPath<'b>,
    ) -> zbus::Result<Self> {
        let screencast_stream_proxy =
            muffin_proxy::ScreenCastStream::new(&connection, stream_object_path).await?;

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
