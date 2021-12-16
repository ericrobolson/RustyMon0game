use m_window::{ControlFlow, Event, EventLoop, WindowEvent};

use crate::{engine::Engine, events::EngineEvent, Game};

pub struct Window {
    width: u32,
    height: u32,
    event_loop: Option<m_window::EventLoop<()>>,
    raw_handle: Option<m_window::Window>,
}
impl Window {
    pub fn new(width: u32, height: u32) -> Self {
        let (event_loop, raw_handle) = m_window::init_window();

        Self {
            width,
            raw_handle: Some(raw_handle),
            event_loop: Some(event_loop),
            height,
        }
    }

    pub(crate) fn raw_handle(&mut self) -> Option<&mut m_window::Window> {
        if let Some(raw) = &mut self.raw_handle {
            Some(raw)
        } else {
            None
        }
    }

    /// Begins execution of the window
    pub(crate) fn begin_execution<TGame>(
        mut engine: crate::engine::Engine,
        game: TGame,
        renderer: crate::renderer::Renderer,
    ) where
        TGame: Game + 'static,
    {
        // If we have a window, we'll want to use that for the loop.
        if let Some(engine_window) = &mut engine.window {
            if let Some(window) = engine_window.raw_handle.take() {
                if let Some(event_loop) = engine_window.event_loop.take() {
                    window_execution(event_loop, window, engine, game, renderer);
                    return;
                }
            }
        }

        // Things didn't work, so we'll just run in a loop.
        Engine::headless_game_loop(engine, game);
    }

    pub(crate) fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}

fn window_execution<'a, TGame>(
    event_loop: EventLoop<()>,
    window: m_window::Window,
    mut engine: crate::engine::Engine,
    mut game: TGame,
    renderer: crate::renderer::Renderer,
) where
    TGame: Game + 'static,
{
    event_loop.run(move |event, _, control_flow| {
        // Process events
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => engine.quit(),
                _ => {}
            },
            _ => {}
        };

        // Map event
        if let Some(event) = map_event(event) {
            engine.on_event(event);
        }

        // Execute updates
        Engine::update(&mut engine, &mut game);

        // Check if we should quit
        if engine.should_exit() {
            *control_flow = ControlFlow::Exit
        }
    });
}

fn map_event(event: Event<()>) -> Option<EngineEvent> {
    None
}
