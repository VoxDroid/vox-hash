use std::sync::atomic::{AtomicBool, Ordering};

static SHUTDOWN: AtomicBool = AtomicBool::new(false);

pub fn set_shutdown() {
    SHUTDOWN.store(true, Ordering::SeqCst);
}

pub fn is_shutdown() -> bool {
    SHUTDOWN.load(Ordering::SeqCst)
}
