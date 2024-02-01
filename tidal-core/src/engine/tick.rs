use std::time::Duration;

use serde::{Deserialize, Serialize};

/// The minimum time interval understood by the engine, using a fixed frequency of 240 Hz.
///
/// A [`Tick`] is internally implemented as an unsigned 32-bits value. This ensures overflow
/// won't be possible in a reasonable life-time.
///
/// Considering the engine is left running non-stop, it would take 207 days to overflow the value.
///
/// Formula: ((2 ^ 32) ÷ 240 ÷ 60 ÷ 60 ÷ 24) = ~207 days
///          u32::MAX     hz    m    h    d
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[repr(transparent)]
pub struct Tick(u32);

impl From<Duration> for Tick {
    fn from(value: Duration) -> Self {
        let tick = (value.as_secs_f32() * 240.0) as u32;

        Self(tick)
    }
}
