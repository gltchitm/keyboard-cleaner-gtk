use gtk::prelude::*;
use gdk::WindowExt;

use crate::constants;
use crate::grab;

fn now() -> u128 {
    let time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH);
    time.unwrap().as_millis()
}

pub struct LockedWindow {
    window: gtk::ApplicationWindow
}

impl LockedWindow {
    pub fn new(app: &gtk::Application) -> Self {
        Self {
            window: gtk::ApplicationWindow::new(app)
        }
    }
    pub fn init(&self) {
        self.window.set_title(constants::MESSAGE_WINDOW_TITLE);
        self.window.set_skip_taskbar_hint(true);
        self.window.set_skip_pager_hint(true);
        self.window.set_decorated(false);
        self.window.set_keep_above(true);
        self.window.set_deletable(false);
        self.window.fullscreen();

        let provider = gtk::CssProvider::new();
        let stylesheet = include_bytes!("stylesheet.css");

        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().unwrap(),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
        );

        provider.load_from_data(stylesheet).unwrap();

        let vertical_box = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let center_label = gtk::Label::new(Some(constants::MESSAGE_KEYBOARD_CLEANER_ACTIVATED));
        let unlock_instructions_label = gtk::Label::new(Some(constants::MESSAGE_HOLD_TO_UNLOCK));

        center_label.set_widget_name("center_label");
        unlock_instructions_label.set_widget_name("unlocked_instructions_label");

        vertical_box.set_center_widget(Some(&center_label));
        vertical_box.pack_end(
            &unlock_instructions_label,
            false,
            false,
            constants::BOTTOM_LABEL_PADDING
        );

        self.window.connect_realize(|window| {
            let cursor = gdk::Cursor::new_for_display(&window.get_display(), gdk::CursorType::BlankCursor);
            window.get_window().unwrap().set_cursor(Some(&cursor));
        });

        self.window.add(&vertical_box);
    }
    pub fn show_and_grab(self) {
        self.window.connect_focus_in_event(|window, _event| {
            let seat = window.get_display().get_default_seat().unwrap();

            let mut grab_result = grab::try_grab(&seat, window);
            let grab_attempt_started_at = now();
            while grab_result.is_err() {
                if now() - grab_attempt_started_at > constants::MAX_GRAB_RETRY_DURATION {
                    panic!("failed to acquire grab!");
                }
                grab_result = grab::try_grab(&seat, window);
            }

            gtk::Inhibit(true)
        });

        let right_mouse_down = std::cell::Cell::new(None);
        self.window.connect_event(move |_window, event| {
            if event.get_event_type() == gdk::EventType::ButtonPress {
                if event.get_button().unwrap() == 3 {
                    right_mouse_down.set(Some(now()));
                    return gtk::Inhibit(true);
                }
            } else if event.get_event_type() == gdk::EventType::ButtonRelease {
                if event.get_button().unwrap() == 3 {
                    if right_mouse_down.get().is_some() {
                        if now() - right_mouse_down.get().unwrap() > constants::HOLD_TO_UNLOCK_DURATION {
                            std::process::exit(0);
                        } else {
                            right_mouse_down.set(None);
                        }
                    }
                }
            }

            gtk::Inhibit(false)
        });

        self.window.show_all();
        self.window.present();
    }
}
