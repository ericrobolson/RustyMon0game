use crate::time::Instant;

/// Gets the delta_t in seconds.
pub(crate) fn get_delta_t(last_execution: &mut Instant) -> f32 {
    let now = Instant::now();
    let delta = now - *last_execution;
    let delta_t = delta.as_secs_f32();
    *last_execution = now;

    delta_t
}
