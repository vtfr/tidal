use std::time::{Duration, SystemTime};

use cgmath::Vector3;
use lazy_static::lazy_static;

use tidal_core_derive::evaluator;

use crate::interpreter::evaluator::EvaluateContext;
use crate::interpreter::evaluator::EvaluateError;
use crate::renderer::CommandList;

#[evaluator(CosEvaluator for Cos)]
#[output(name = "cosine")]
pub(crate) fn evaluate_cos(value: f32) -> f32 {
    value.cos()
}

#[evaluator(SinEvaluator for Sin)]
#[output(name = "sine")]
pub(crate) fn evaluate_sin(value: f32) -> f32 {
    value.sin()
}

#[evaluator(ComposeVectorEvaluator for ComposeVector)]
#[output(name = "vector")]
pub(crate) fn evaluate_compose_vector(x: f32, y: f32, z: f32) -> Vector3<f32> {
    Vector3::new(x, y, z)
}

#[evaluator(TimeEvaluator for Time)]
#[output(name = "time")]
pub(crate) fn evaluate_time() -> f32 {
    lazy_static! {
        static ref START: SystemTime = SystemTime::now() - Duration::from_secs(1);
    }

    SystemTime::now()
        .duration_since(*START)
        .unwrap()
        .as_secs_f32()
}

#[evaluator(RemapEvaluator for Remap)]
#[output(name = "value")]
pub(crate) fn evaluate_remap(
    value: f32,
    from_min: f32,
    from_max: f32,
    to_min: f32,
    to_max: f32,
) -> f32 {
    let c = ((value - from_min) / (from_max - from_min)).clamp(0.0, 1.0);

    to_min + c * (to_max - to_min)
}
