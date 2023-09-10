use gtk::prelude::*;
use gtk::Application;
use gtk::ApplicationWindow;
use gtk::glib;

const APP_ID: &str = "com.aagrh111.Resistor";

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title(env!("CARGO_PKG_NAME"))
        .build();
    window.present();
}
