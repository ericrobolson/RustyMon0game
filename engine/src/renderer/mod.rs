use crate::*;
use time::Instant;
pub struct Renderer {
    last_execution: Instant,
}
impl Renderer {
    /// Creates a new renderer
    pub(crate) fn new() -> Self {
        Self {
            last_execution: Instant::now(),
        }
    }

    /// Renders the game and engine
    pub(crate) fn render<TGame>(renderer: &mut Self, game: &mut TGame)
    where
        TGame: Game,
    {
        let delta_t = util::get_delta_t(&mut renderer.last_execution);
        game.render(delta_t, renderer);
    }
}
