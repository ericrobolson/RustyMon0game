pub struct Window {
    width: u32,
    height: u32,
}
impl Window {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub(crate) fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}
