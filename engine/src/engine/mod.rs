use crate::*;

use file_system::*;
use input::*;
use services::*;
use time::*;
use window::*;

/// Contains all functionality the engine will provide outside of rendering.
pub struct Engine {
    pub input: Input,
    pub services: Services,
    pub file_system: FileSystem,
    pub window: Window,
    pub(crate) should_exit: bool,

    last_update: Instant,
}

impl Engine {
    /// Creates a new instance of the engine.
    pub(crate) fn new() -> Self {
        Self {
            input: Input::new(),
            services: Services::new(),
            file_system: FileSystem::new(),
            window: Window::new(0, 0),
            should_exit: false,
            last_update: Instant::now(),
        }
    }

    /// Handles the given event.
    pub(crate) fn on_event(&mut self, event: EngineEvent) {
        match &event {
            EngineEvent::WindowResize {
                width_px,
                height_px,
            } => self.window.set_size(*width_px, *height_px),
            EngineEvent::Shutdown => self.should_exit = true,
        }

        self.input.on_event(event);
    }

    /// Updates the game and engine
    pub(crate) fn update<TGame>(engine: &mut Self, game: &mut TGame)
    where
        TGame: Game,
    {
        let delta_t = util::get_delta_t(&mut engine.last_update);
        game.update(delta_t, engine);
    }

    /// Signals that the engine should quit.
    pub fn quit(&mut self) {
        self.should_exit = true;
    }
}
