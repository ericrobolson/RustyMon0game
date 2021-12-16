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
    pub window: Option<Window>,
    should_exit: bool,
    last_update: Instant,
}

impl Engine {
    /// Creates a new instance of the engine.
    pub(crate) fn new(use_window: bool) -> Self {
        Self {
            input: Input::new(),
            services: Services::new(),
            file_system: FileSystem::new(),
            window: if use_window {
                Some(Window::new(0, 0))
            } else {
                None
            },
            should_exit: false,
            last_update: Instant::now(),
        }
    }

    /// Begins execution of the engine
    pub(crate) fn begin_execution<TGame>()
    where
        TGame: Game + 'static,
    {
        let mut game = TGame::new();
        let mut engine = engine::Engine::new(true);

        let renderer = match &mut engine.window {
            Some(window) => Some(renderer::Renderer::new(window)),
            None => None,
        };

        game.initialize(&mut engine);

        match renderer {
            Some(renderer) => window::Window::begin_execution(engine, game, renderer),
            None => Self::headless_game_loop(engine, game),
        }
    }

    /// Handles the given event.
    pub(crate) fn on_event(&mut self, event: EngineEvent) {
        match &event {
            EngineEvent::WindowResize {
                width_px,
                height_px,
            } => {
                if let Some(window) = &mut self.window {
                    window.set_size(*width_px, *height_px)
                }
            }
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

    /// Returns whether the engine should exit or not
    pub(crate) fn should_exit(&self) -> bool {
        self.should_exit
    }

    /// Signals that the engine should quit.
    pub fn quit(&mut self) {
        self.should_exit = true;
    }

    /// Runs the game in a headless game loop
    pub(crate) fn headless_game_loop<TGame>(mut engine: crate::engine::Engine, mut game: TGame)
    where
        TGame: Game + 'static,
    {
        while !engine.should_exit {
            Self::update(&mut engine, &mut game);
        }
    }
}
