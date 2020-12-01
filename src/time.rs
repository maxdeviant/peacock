use std::time::Duration;

use crate::PeacockContext;

pub fn duration_to_f64(duration: Duration) -> f64 {
    let seconds = duration.as_secs() as f64;
    let nanos = f64::from(duration.subsec_nanos()) * 1e-9;
    seconds + nanos
}

pub fn f64_to_duration(duration: f64) -> Duration {
    debug_assert!(duration >= 0.0);
    let seconds = duration.trunc() as u64;
    let nanos = (duration.fract() * 1e9) as u32;
    Duration::new(seconds, nanos)
}

pub fn get_fps(ctx: &PeacockContext) -> f64 {
    ctx.fps_tracker.fps()
}
