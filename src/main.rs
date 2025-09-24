mod log;
mod settings;
mod scan;
mod ui;

fn main() -> iced::Result {
    log::setup();
    ui::setup()
}
