struct HelloWorld;

impl cone::App for HelloWorld {
    fn draw(&self, frame: &mut cone::Frame) {
        let text = "Hello world!";
        frame.clear();
        frame.put_str((frame.width() - text.len() as u32) / 2, frame.height() / 2, text, cone::WHITE)
    }

    fn update(&mut self) {}
}

fn main() -> cone::Result<()> {
    let app = HelloWorld;
    cone::run(cone::Config {
        title: "Hello world!",
        width: 80,
        height: 50,
        font_path: "examples/Alloy_curses_12x12.png",
    },
    app)
}
