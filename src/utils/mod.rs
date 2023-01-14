use std::sync::atomic::Ordering;
use std::sync::{atomic::AtomicBool, Arc, Mutex};

pub mod config;
pub mod files;

pub struct AsyncBool {
    value: Arc<Mutex<AtomicBool>>,
}

impl AsyncBool {
    pub fn init(v: bool) -> Self {
        Self {
            value: Arc::new(Mutex::new(AtomicBool::new(v))),
        }
    }

    pub fn set(&self, val: bool) -> () {
        self.value.lock().unwrap().store(val, Ordering::SeqCst);
    }

    pub fn get(&self) -> bool {
        let arc_clone = self.value.clone();
        let b = arc_clone.lock().unwrap().load(Ordering::SeqCst);
        b
    }
}
