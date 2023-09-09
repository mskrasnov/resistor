use resistor::tui;

fn main() {
    let mut scr = cursive::default();
    tui::main_window(&mut scr);
    scr.run();
}
