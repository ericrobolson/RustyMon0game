use core::ops::Sub;
use std::time;

#[derive(Copy, Clone)]
pub struct Instant {
    inner: time::Instant,
}

impl Instant {
    pub fn now() -> Self {
        Self {
            inner: time::Instant::now(),
        }
    }
}

impl Sub for Instant {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Duration {
        Duration {
            inner: self.inner - rhs.inner,
        }
    }
}

pub struct Duration {
    inner: time::Duration,
}
impl Duration {
    pub fn as_secs_f32(&self) -> f32 {
        self.inner.as_secs_f32()
    }
}
