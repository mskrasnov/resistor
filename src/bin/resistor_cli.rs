use resistor::tui;
use libmsg::tui::Tui;
use libmsg::tui::Release;

fn main() {
    print!("{}-v{} ", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("{}", env!("CARGO_PKG_DESCRIPTION"));
    println!("Copyright (C) 2023 {}", env!("CARGO_PKG_AUTHORS"));

    let mut scr = Tui::new(Release::detect_release_type(env!("CARGO_PKG_VERSION")))
        .init()
        .cursive;

    tui::main_window(&mut scr);
    scr.run();
}
