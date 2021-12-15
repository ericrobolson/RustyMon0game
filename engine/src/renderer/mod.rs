use crate::*;
use mem::*;
use time::Instant;

/// A basic renderer abstraction.
pub struct Renderer {
    last_execution: Instant,
    queued_commands: StaticBuffer<RenderCommand>,
    gfx_renderer: Option<Box<dyn GfxRenderer>>,
}
impl Renderer {
    /// Creates a new renderer
    pub(crate) fn new() -> Self {
        let max_commands = 1000;

        Self {
            gfx_renderer: None,
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
                        screen_position,
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

/// The actual hardware renderer.
/// This is extremely simple to ensure ease of porting.
pub trait GfxRenderer {
    /// Starts a new render pass.
    fn begin_render_pass(&mut self);
    /// Ends the render pass
    fn end_render_pass(&mut self);
    /// Presents the image
    fn render(&mut self);

    /// Drops the given image from the GPU.
    fn drop_image(&mut self, image_id: ());
    /// Pushes the given image to the GPU.
    fn load_image(&mut self, image_id: (), image: ());

    /// Queues the given image to be rendered.
    fn queue_image(
        &mut self,
        image_id: (),
        screen_position: (RangeN1to1, RangeN1to1, RangeN1to1),
        scale: (f32, f32),
        sub_img: (Range0to1, Range0to1),
        color: Color,
    );
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
