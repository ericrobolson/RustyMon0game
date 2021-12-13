/// Internal representation of engine events.
/// Can be things like window resizing, joystick changes, etc.
pub(crate) enum EngineEvent {
    Shutdown,
    WindowResize { width_px: u32, height_px: u32 },
}
