struct Keyboard {
    text: String,
}

impl cone::App for Keyboard {
    fn draw(&self, frame: &mut cone::Frame) {
        frame.clear();
        frame.put_str(0, 0, &self.text, cone::Color::WHITE)
    }

    fn update(&mut self, update: &cone::Update) {
        if update.is_pressed(cone::KeyCode::Space) {
            self.text.push('@');
        }
    }
}

fn main() -> cone::Result<()> {
    let app = Keyboard {
        text: String::new(),
    };
    cone::run(cone::Config {
        title: "Hello world!",
        width: 80,
        height: 50,
        font: include_bytes!("Alloy_curses_12x12.png"),
    },
    app)
}
