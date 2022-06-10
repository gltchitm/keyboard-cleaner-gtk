use gtk::prelude::*;

use crate::constants;
use crate::grab;

use std::cell::Cell;

fn now() -> u128 {
    let time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH);
    time.unwrap().as_millis()
}

pub struct LockedWindow {
    window: gtk::ApplicationWindow,
}

impl LockedWindow {
    pub fn new(app: &gtk::Application) -> Self {
        Self {
            window: gtk::ApplicationWindow::new(app),
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
            &gdk::Screen::default().unwrap(),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
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
            constants::BOTTOM_LABEL_PADDING,
        );

        self.window.connect_realize(|window| {
            let cursor = gdk::Cursor::for_display(&window.display(), gdk::CursorType::BlankCursor);
            window.window().unwrap().set_cursor(Some(&cursor.unwrap()));
        });

        self.window.add(&vertical_box);
    }

    pub fn show_and_grab(self) {
        self.window.connect_focus_in_event(|window, _event| {
            let seat = window.display().default_seat().unwrap();

            let mut grab_result = grab::try_grab(&seat, window);
            let grab_attempt_started_at = now();
            while grab_result.is_err() {
                if now() - grab_attempt_started_at > constants::MAX_GRAB_RETRY_DURATION {
                    panic!("failed to acquire grab!");
                }
                grab_result = grab::try_grab(&seat, window);
                if grab_result.is_err() {
                    std::thread::sleep(std::time::Duration::from_millis(150));
                }
            }

            gtk::Inhibit(true)
        });

        let right_mouse_down = Cell::new(None);
        self.window.connect_event(move |_window, event| {
            if event.event_type() == gdk::EventType::ButtonPress {
                if event.button().unwrap() == 3 {
                    right_mouse_down.replace(Some(now()));
                    return gtk::Inhibit(true);
                }
            } else if event.event_type() == gdk::EventType::ButtonRelease {
                if event.button().unwrap() == 3 {
                    if let Some(is_right_mouse_down) = right_mouse_down.get() {
                        if now() - is_right_mouse_down > constants::HOLD_TO_UNLOCK_DURATION {
                            std::process::exit(0);
                        } else {
                            right_mouse_down.replace(None);
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
