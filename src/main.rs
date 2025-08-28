mod tui;
mod colors;

fn main() {
    print!("{}-v{} ", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("{}", env!("CARGO_PKG_DESCRIPTION"));
    println!("Copyright (C) 2023-2025 {}", env!("CARGO_PKG_AUTHORS"));

    let mut scr = cursive::default();

    tui::main_window(&mut scr);
    scr.run();
}
