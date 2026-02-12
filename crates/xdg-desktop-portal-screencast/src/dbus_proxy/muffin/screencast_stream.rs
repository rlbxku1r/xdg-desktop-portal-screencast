#[zbus::proxy(
    interface = "org.cinnamon.Muffin.ScreenCast.Stream",
    default_service = "org.cinnamon.Muffin.ScreenCast",
    gen_blocking = false
)]
pub trait Stream {
    #[zbus(signal, name = "PipeWireStreamAdded")]
    fn pipewire_stream_added(&self, node_id: u32) -> zbus::Result<()>;

    #[zbus(property)]
    fn parameters(
        &self,
    ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;
}
