use std::sync::{Arc, LockResult, Mutex, MutexGuard};

use tidal_core::demo::Demo;
use tidal_core::interpreter::Interpreter;

#[derive(Clone)]
pub struct InterpreterHolder(Arc<Mutex<Interpreter>>);

impl InterpreterHolder {
    #[inline]
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(Interpreter::default())))
    }

    pub fn lock(&self) -> MutexGuard<'_, Interpreter> {
        self.0.lock().unwrap()
    }

    pub fn load_demo(&mut self, demo: Demo) {
        let mut guard = self.0.lock().expect("mutex unlock");

        *guard = Interpreter::new(demo)
    }
}
