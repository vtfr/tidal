#![feature(downcast_unchecked)]

extern crate core;

pub mod app;
pub mod engine;
pub mod graph;
pub mod macros;
pub mod math;
pub mod operator;
pub mod resource;

pub mod prelude {
    pub use app::App;
    pub use math::Quaternion;
    pub use math::Vector2;
    pub use math::Vector3;

    use crate::{app, engine, math};
}
