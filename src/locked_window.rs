extern crate gdk;

use super::gtk::prelude::*;
use super::constants;
use super::grab;
use super::now;

pub struct LockedWindow {
    window: gtk::ApplicationWindow
}

impl LockedWindow {
    pub fn new(app: &gtk::Application) -> LockedWindow {
        LockedWindow {
            window: gtk::ApplicationWindow::new(app)
        }
    }
    pub fn init(&'_ self) {
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
        
        gtk::WidgetExt::set_widget_name(&center_label, "center_label");
        gtk::WidgetExt::set_widget_name(&unlock_instructions_label, "unlocked_instructions_label");

        vertical_box.set_center_widget(Some(&center_label));
        vertical_box.pack_end(&unlock_instructions_label, false, false, constants::BOTTOM_LABEL_PADDING);

        self.window.connect_realize(|window| {
            let cursor = gdk::Cursor::new_for_display(&window.get_display(), gdk::CursorType::BlankCursor);
            gdk::WindowExt::set_cursor(&window.get_window().unwrap(), Some(&cursor));
        });

        self.window.add(&vertical_box);
    }
    pub fn show_and_grab(self) {
        self.window.connect_focus_in_event(|window, _event| {
            let seat = window.get_display().get_default_seat().unwrap();
            grab::grab_or_panic(seat, window);

            return gtk::Inhibit(true);
        });

        let right_mouse_down = std::cell::Cell::new(None);
        self.window.connect_event(move |_window, event| {
            if event.get_event_type() == gdk::EventType::ButtonPress {
                if event.get_button().unwrap() == 3 {
                    right_mouse_down.set(Some(now::now()));
                    return gtk::Inhibit(true);
                }
            } else if event.get_event_type() == gdk::EventType::ButtonRelease {
                if event.get_button().unwrap() == 3 {
                    if right_mouse_down.get().is_some() {
                        if now::now() - right_mouse_down.get().unwrap() > constants::HOLD_TO_UNLOCK_DURATION {
                            std::process::exit(0);
                        } else {
                            right_mouse_down.set(None);
                        }
                    }
                }
            }

            return gtk::Inhibit(false);
        });

        self.window.show_all();
        self.window.present();
    }
}
