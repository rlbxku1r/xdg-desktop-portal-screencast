use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

pub fn setup() -> Arc<AtomicBool> {
    let sigint_caught = Arc::new(AtomicBool::new(false));
    let sigint_caught_1 = sigint_caught.clone();
    tokio::spawn(async move {
        if let Err(err) = tokio::signal::ctrl_c().await {
            log::error!("SIGINT: {err}");
            return;
        }
        sigint_caught_1.store(true, Ordering::Relaxed);
    });
    sigint_caught
}
