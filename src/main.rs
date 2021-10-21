use gio::prelude::*;

mod constants;
mod grab;
mod locked_window;
mod now;

fn main() {
    #[cfg(not(target_os = "linux"))]
    compile_error!("only Linux is supported!");

    let application = gtk::Application::new(None, Default::default()).unwrap();
    application.connect_activate(|app| {
        let window = locked_window::LockedWindow::new(app);
        window.init();
        window.show_and_grab();
    });

    application.run(&[]);
}
