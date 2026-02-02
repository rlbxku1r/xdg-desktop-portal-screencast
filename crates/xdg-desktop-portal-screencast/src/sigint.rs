use std::sync::Mutex;

static CALLBACK: Mutex<Option<Box<dyn FnMut() + Send>>> = Mutex::new(None);

pub fn set_callback<C>(callback: C)
where
    C: FnMut() + Send + 'static,
{
    if let Ok(mut x) = CALLBACK.lock() {
        assert!(x.is_none(), "signal handlers can be set only once");
        *x = Some(Box::new(callback));
        unsafe {
            libc::signal(libc::SIGINT, c_callback as *const () as libc::sighandler_t);
        }
    }
}

extern "C" fn c_callback(_: libc::c_int) {
    if let Ok(mut x) = CALLBACK.lock()
        && let Some(ref mut callback) = *x
    {
        callback();
    }
}
