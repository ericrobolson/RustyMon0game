use engine::{engine::*, file_system::FileLoad, renderer::Renderer};
pub struct Game {}

impl engine::Game for Game {
    fn new() -> Self {
        Self {}
    }

    fn initialize(&mut self, engine: &mut Engine) {
        let file_contents = match engine.file_system.load("test.txt") {
            Ok(bytes) => bytes,
            Err(e) => todo!("{:?}", e),
        };

        match file_contents {
            FileLoad::Loading => todo!(),
            FileLoad::File(f) => {
                let str = std::str::from_utf8(&f).unwrap();
                println!("{}", str);
            }
        }
    }

    fn update(&mut self, _delta_t: f32, _engine: &mut Engine) {}

    fn render(&mut self, _delta_t: f32, _renderer: &mut Renderer) {}
}
