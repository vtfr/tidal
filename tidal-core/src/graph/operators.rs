use std::borrow::Cow;

use crate::graph::Metadata;
use crate::interpreter::Evaluate;

//
// #[derive(Serialize, Deserialize, Debug, Describe, Clone, Prototypes)]
// pub enum Operator {
//     #[output("time", Scalar)]
//     Time,
//
//     #[input("value", Scalar)]
//     #[output("sin", Scalar)]
//     Sin,
//
//     /// Calculate the cosine of a value
//     #[input("value", Scalar)]
//     #[output("cos", Scalar)]
//     Cos,
//
//     #[input("value", Scalar)]
//     #[input("from.min", Scalar)]
//     #[input("from.max", Scalar)]
//     #[input("to.min", Scalar)]
//     #[input("to.max", Scalar)]
//     #[output("value", Scalar)]
//     Remap,
//
//     #[input("objects", Command)]
//     #[input("eye", Vector, default(0.0, 0.0, - 1.0))]
//     #[input("target", Vector, default(0.0, 0.0, 0.0))]
//     #[input("up", Vector, default(0.0, 1.0, 0.0))]
//     #[input("aspect", Scalar, default(1.77))]
//     #[input("fov", Scalar, default(45.0))]
//     #[input("z-near", Scalar, default(0.1))]
//     #[input("z-far", Scalar, default(1000.0))]
//     #[output("camera", Command)]
//     Camera,
//
//     #[input("x", Scalar)]
//     #[input("y", Scalar)]
//     #[input("z", Scalar)]
//     #[output("vector", Vector)]
//     ComposeVector,
//
//     #[output("mesh", Mesh)]
//     Triangle,
//
//     #[output("mesh", Texture)]
//     #[output("value", Command)]
//     DrawTexture,
//
//     #[input("command", Command)]
//     #[output("render", Command)]
//     PrincipalPass,
//
//     #[input("scene", Command, multiple)]
//     Scene,
// }
