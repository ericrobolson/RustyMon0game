pub struct Game {}

impl engine::Game for Game {
    fn new() -> Self {
        Self {}
    }

    fn initialize(&mut self, engine: &mut engine::Engine) {
        let file_contents = match engine.file_system.load("test.txt") {
            Ok(bytes) => bytes,
            Err(e) => todo!("{:?}", e),
        };

        match file_contents {
            engine::FileLoad::Loading => todo!(),
            engine::FileLoad::File(f) => {
                let str = std::str::from_utf8(&f).unwrap();
                println!("{}", str);
            }
        }
    }

    fn update(&mut self, _delta_t: f32, _engine: &mut engine::Engine) {}

    fn render(&mut self, _delta_t: f32, _renderer: &mut engine::Renderer) {}
}
