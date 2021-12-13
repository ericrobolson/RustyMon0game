use core::ops::Sub;

#[derive(Copy, Clone)]
pub struct Instant {}

impl Instant {
    pub fn now() -> Self {
        todo!()
    }
}

impl Sub for Instant {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Duration {
        todo!()
    }
}

pub struct Duration {}
impl Duration {
    pub fn as_secs_f32(&self) -> f32 {
        todo!()
    }
}
