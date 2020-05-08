struct HelloWorld;

impl cone::App for HelloWorld {
    fn draw(&self, frame: &mut cone::Frame) {
        let text = "Hello world!";
        frame.clear();
        frame.put_str((frame.width() - text.len() as i32) / 2, frame.height() / 2, text, cone::Color::WHITE)
    }

    fn update(&mut self, _: &cone::Update) {}
}

fn main() -> cone::Result<()> {
    let app = HelloWorld;
    cone::run(cone::Config {
        title: "Keyboard",
        width: 80,
        height: 50,
        font: include_bytes!("Alloy_curses_12x12.png"),
    },
    app)
}
