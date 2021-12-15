#![cfg_attr(not(test), no_std)]
#![macro_use]
extern crate alloc;

pub mod mem;

pub mod engine;
pub mod events;
pub mod file_system;
pub mod input;
pub mod renderer;
pub mod services;
pub mod time;
pub mod util;
pub mod window;

pub(crate) use events::*;

/// A set of traits the game must implement for usage by the engine.
/// Monogame inspired.
pub trait Game: Sized {
    /// This method constructs the game.
    fn new() -> Self;

    /// This method is called after `new` but before the main game loop.
    /// This is where you can query the services and load content.
    ///
    /// `engine` is a handle to the engine.
    fn initialize(&mut self, engine: &mut engine::Engine);

    /// This method is called multiple times per second, and is used to update your
    /// game state (checking for collisions, gathering input, playing audio, etc.).
    ///
    /// `delta_t` is the time elapsed since the last call in seconds.
    ///
    /// `engine` is a handle to the engine.
    fn update(&mut self, delta_t: f32, engine: &mut engine::Engine);

    /// This method is called multiple times per second and is used to render graphics
    /// or play audio.
    ///
    /// `delta_t` is the time elapsed since the last call in seconds.
    ///
    /// `renderer` is a handle to the renderer.
    fn render(&mut self, delta_t: f32, renderer: &mut renderer::Renderer);
}

/// Begins execution of the given game.
pub fn init<TGame>()
where
    TGame: Game + 'static,
{
    // Set up everything
    let mut game = TGame::new();
    let mut engine = engine::Engine::new();
    let renderer = renderer::Renderer::new();

    // Initialize game
    game.initialize(&mut engine);
}

#[cfg(target_arch = "wasm32")]
pub mod web {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        pub fn alert(s: &str);
    }
}

/// Executes the engine. Using a macro to enable web.
#[macro_export]
macro_rules! execute {
    ($game:ty) => {
        fn main() {
            engine::init::<$game>();
        }

        // Load up web dependencies
        #[cfg(target_arch = "wasm32")]
        use wasm_bindgen::prelude::*;

        // Run the web dependencies
        #[cfg(target_arch = "wasm32")]
        #[wasm_bindgen]
        pub fn run() {
            engine::web::alert("Hello world!");

            main();
        }
    };
}
