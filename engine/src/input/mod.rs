use crate::EngineEvent;

pub struct Input {}
impl Input {
    pub(crate) fn new() -> Self {
        Self {}
    }

    /// Handles the given event.
    pub(crate) fn on_event(&mut self, event: EngineEvent) {
        match event {
            EngineEvent::WindowResize { .. } => {}
            EngineEvent::Shutdown => {}
        }
    }
}
