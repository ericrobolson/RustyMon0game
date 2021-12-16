mod gpu_wgpu;

/// Creates a new gpu instance
pub fn init_gpu(window: &m_window::Window) -> Box<dyn Gpu> {
    Box::new(gpu_wgpu::GpuWgpu::new(window))
}

/// Hardware GPU interface.
pub trait Gpu {
    /// Starts a new render pass.
    fn begin_render_pass(&mut self);
    /// Ends the render pass
    fn end_render_pass(&mut self);
    /// Presents the image
    fn render(&mut self);
}
