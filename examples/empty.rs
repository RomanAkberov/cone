struct Empty;

impl cone::App for Empty {
    fn draw(&self, _: &mut cone::Frame) {}

    fn update(&mut self, _: &cone::Update) {}
}

fn main() -> cone::Result<()> {
    let app = Empty;
    cone::run(cone::Config {
        title: "Empty",
        width: 80,
        height: 50,
        font: include_bytes!("Alloy_curses_12x12.png"),
    },
    app)
}
