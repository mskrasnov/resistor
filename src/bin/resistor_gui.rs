use resistor::gui::ui;

fn main() -> iced::Result {
    print!("{}-v{} ", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("{}", env!("CARGO_PKG_DESCRIPTION"));
    println!("Copyright (C) 2023, 2024 {}", env!("CARGO_PKG_AUTHORS"));

    ui()
}
