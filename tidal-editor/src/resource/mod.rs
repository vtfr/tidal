use std::sync::{Arc, Mutex, MutexGuard};

use derive_more::{Constructor, Into};

// #[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Constructor, Into)]
// pub struct ResourceName(pub String);
//
// #[derive(Debug)]
// pub struct ResourceManager {
//     resources: Arc<Mutex<HashMap<ResourceName, Contents>>>,
// }
//
// impl ResourceManager {
//     pub fn get_resource(&self) {
//         let guard = self.resources.lock().unwrap();
//     }
// }
//
// #[derive(Debug, Clone, PartialOrd, PartialEq, Eq)]
// pub enum Contents {
//     Shader(String),
// }
