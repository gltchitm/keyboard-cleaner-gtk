extern crate gtk;
extern crate gio;

use gio::prelude::*;

mod locked_window;
mod constants;
mod grab;
mod now;

fn main() {
    let application = gtk::Application::new(
        None,
        Default::default()
    ).unwrap();
    application.connect_activate(|app| {
        let window = locked_window::LockedWindow::new(app);
        window.init();
        window.show_and_grab();
    });

    application.run(&[]);
}
