use crate::{window::Window, *};
use m_gpu::{init_gpu, Gpu};
use mem::*;
use time::Instant;

/// A basic renderer abstraction.
pub struct Renderer {
    gfx_renderer: Option<Box<dyn Gpu>>,
    last_execution: Instant,
    queued_commands: StaticBuffer<RenderCommand>,
}
impl Renderer {
    /// Creates a new renderer
    pub(crate) fn new(window: &mut Window) -> Self {
        let max_commands = 1000;

        let gfx_renderer = match window.raw_handle() {
            Some(handle) => Some(init_gpu(handle)),
            None => None,
        };

        Self {
            gfx_renderer,
            last_execution: Instant::now(),
            queued_commands: StaticBuffer::new(max_commands),
        }
    }

    /// Queues up the given command for execution
    pub fn queue(&mut self, render_command: RenderCommand) -> Result<(), RenderError> {
        match self.queued_commands.push(render_command) {
            Ok(_) => Ok(()),
            Err(e) => match e {
                BufferError::BufferFull => Err(RenderError::QueueFull),
            },
        }
    }

    /// Renders the game and engine
    pub(crate) fn render<TGame>(renderer: &mut Self, game: &mut TGame)
    where
        TGame: Game,
    {
        let can_render = renderer.gfx_renderer.is_some();

        if can_render {
            let delta_t = util::get_delta_t(&mut renderer.last_execution);
            game.render(delta_t, renderer);
        }

        if let Some(gfx) = &mut renderer.gfx_renderer {
            gfx.begin_render_pass();

            // TODO: actually do the things
            while let Some(command) = renderer.queued_commands.pop_head() {
                match command {
                    RenderCommand::Image {
                        image_id,
                        screen_position_px,
                        scale,
                        sub_img,
                    } => todo!(),
                    RenderCommand::LoadImage { image_id, image } => todo!(),
                    RenderCommand::DropImage { image_id } => todo!(),
                }
            }

            gfx.end_render_pass();
        }
    }
}

/// RGBA color
pub struct Color([u8; 4]);

pub enum RenderCommand {
    Image {
        image_id: (),
        /// Screen position in pixels. X,Y,Z
        screen_position_px: (i32, i32, u32),
        scale: (f32, f32),
        sub_img: (Range0to1, Range0to1),
    },
    LoadImage {
        image_id: (),
        image: (),
    },
    DropImage {
        image_id: (),
    },
}

/// A range between zero and one.
pub struct Range0to1;
impl Range0to1 {
    pub fn to_f32(&self) -> f32 {
        todo!()
    }
}
impl Numeric for Range0to1 {
    const IS_DETERMINISTIC: bool = false;
}

/// A range from -1.0 to 1.0
pub struct RangeN1to1;
impl RangeN1to1 {
    pub fn to_f32(&self) -> f32 {
        todo!()
    }
}
impl Numeric for RangeN1to1 {
    const IS_DETERMINISTIC: bool = false;
}

/// A generic numberic value.
pub trait Numeric {
    const IS_DETERMINISTIC: bool;
}

pub enum RenderError {
    QueueFull,
}
