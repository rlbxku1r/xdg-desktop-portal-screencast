use std::sync::atomic::{AtomicBool, Ordering};

static SIGNALED: AtomicBool = AtomicBool::new(false);

pub fn setup_handler() {
    extern "C" fn handler(_: libc::c_int) {
        SIGNALED.store(true, Ordering::Relaxed);
    }
    unsafe {
        libc::signal(libc::SIGINT, handler as *const () as libc::sighandler_t);
    }
}

pub fn is_signaled() -> bool {
    SIGNALED.load(Ordering::Relaxed)
}
