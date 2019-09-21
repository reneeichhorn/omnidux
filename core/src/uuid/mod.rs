use std::sync::atomic::{AtomicUsize, Ordering};

static GLOBAL_UUID_COUNTER: AtomicUsize = AtomicUsize::new(0usize);

pub fn gen_uuid() -> usize {
  GLOBAL_UUID_COUNTER.fetch_add(1, Ordering::SeqCst)
}