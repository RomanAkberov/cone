struct Mouse {
    pub pos: (i32, i32),
}

impl cone::App for Mouse {
    fn draw(&self, frame: &mut cone::Frame) {
        frame.clear();
        frame.put_char(self.pos.0, self.pos.1, '@', cone::Color::WHITE);
    }

    fn update(&mut self, update: &cone::Update) {
        self.pos = update.mouse_pos();
    }
}

fn main() -> cone::Result<()> {
    let app = Mouse {
        pos: (0, 0)
    };
    cone::run(cone::Config {
        title: "Keyboard",
        width: 80,
        height: 50,
        font: include_bytes!("Alloy_curses_12x12.png"),
    },
    app)
}
