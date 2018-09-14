//use core::fmt::Debug;
//
// #[derive(Copy, Clone, Debug)]
// pub struct KeyFrame<V> {
//     pub x: Frame,
//     pub y: V,
//     pub i: Interpolation,
// }
//
// #[derive(Clone, Debug)]
// pub struct Animation<V> (Vec<KeyFrame<V>>);
//
// #[repr(transparent)]
// #[derive(Copy, Clone)]
// #[derive(Debug, Into, From, Ord, PartialOrd, Eq, PartialEq, Sub, Add)]
// pub struct Frame(pub(crate) u32);
//
// impl Frame {
//     const MIN: Frame = Frame(u32::MIN);
//     const MAX: Frame = Frame(u32::MAX);
//
//     #[inline(always)]
//     pub fn from_milliseconds(ms: f32) -> Frame {
//         Frame(((ms * 120.0) / 1000.0) as u32)
//     }
// }
//
// #[derive(Copy, Clone, Debug)]
// pub enum Interpolation {
//     Linear,
//     /// Special interpolation that only emits the start value
//     Start,
//     /// Special interpolation that only emits the end value
//     End,
// }
//
// /// How an interpolation should be performed between two points.
// pub trait Interpolate {
//     fn interpolate(interpolation: Interpolation, a: &Self, b: &Self, control: f32) -> Self;
// }
//
// impl Interpolate for f32 {
//     #[inline(always)]
//     fn interpolate(interpolation: Interpolation, a: &Self, b: &Self, control: f32) -> Self {
//         match interpolation {
//             Interpolation::Linear => a + (b - a) * control,
//             Interpolation::Start => *a,
//             Interpolation::End => *b,
//         }
//     }
// }
//
// impl Interpolate for Vector3 {
//     fn interpolate(interpolation: Interpolation, a: &Self, b: &Self, control: f32) -> Self {
//         let x = f32::interpolate(interpolation, &a.x, &b.x, control);
//         let y = f32::interpolate(interpolation, &a.y, &b.y, control);
//         let z = f32::interpolate(interpolation, &a.z, &b.z, control);
//
//         Vec3::new(x, y, z)
//     }
// }
//
// impl Interpolate for Vector2 {
//     fn interpolate(interpolation: Interpolation, a: &Self, b: &Self, control: f32) -> Self {
//         let x = f32::interpolate(interpolation, &a.x, &b.x, control);
//         let y = f32::interpolate(interpolation, &a.y, &b.y, control);
//
//         Vec2::new(x, y)
//     }
// }
//
// pub trait Animate {
//     type Output;
//
//     fn animate(&self, time: Frame) -> Self::Output;
// }
//
// impl<V> Animation<V>
//     where V: Interpolate + Copy + Default {
//     pub fn from_constant(v: V) -> Self {
//         Animation(vec![
//             KeyFrame { x: Frame::MIN, y: v, i: Interpolation::Start },
//             KeyFrame { x: Frame::MAX, y: v, i: Interpolation::Start },
//         ])
//     }
//
//     pub fn from_key_frames(mut key_frames: Vec<KeyFrame<V>>) -> Self {
//         if key_frames.len() == 0 {
//             Self::from_constant(Default::default())
//         } else {
//             // Add two dummy key frames repeating the minimum and maximum values. Will be used
//             // when the animation starts after or ends before the demo's duration.
//             let start = KeyFrame { x: Frame::MIN, y: key_frames.first().unwrap().y, i: Interpolation::End };
//             let end = KeyFrame { x: Frame::MAX, y: key_frames.last().unwrap().y, i: Interpolation::End };
//
//             key_frames.insert(0, start);
//             key_frames.push(end);
//
//             Animation(key_frames)
//         }
//     }
// }
//
// impl<V> Animate for Animation<V>
//     where V: Interpolate {
//     type Output = V;
//
//     fn animate(&self, time: Frame) -> Self::Output {
//         let (current, next) = self.0.windows(2)
//             .map(|w| (&w[0], &w[1]))
//             .find(|(current, next)| {
//                 next.x > time
//             })
//             .unwrap(); // Guaranteed to be exist because of the constructor
//
//         let control = ((time - current.x).0 as f32) / ((next.x - current.x).0 as f32);
//         let control = control.clamp(0.0, 1.0);
//
//         V::interpolate(Interpolation::Linear, &current.y, &next.y, control)
//     }
// }
