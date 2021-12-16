use crate::Gpu;

pub struct GpuWgpu {}

impl GpuWgpu {
    pub fn new(window: &m_window::Window) -> Self {
        Self {}
    }
}

impl Gpu for GpuWgpu {
    fn begin_render_pass(&mut self) {
        todo!()
    }

    fn end_render_pass(&mut self) {
        todo!()
    }

    fn render(&mut self) {
        todo!()
    }
}
