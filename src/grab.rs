use super::gtk::prelude::*;

pub fn grab_or_panic(seat: gdk::Seat, window: &gtk::ApplicationWindow) {
    let grab_status = seat.grab(
        &window.get_window().unwrap(),
        gdk::SeatCapabilities::ALL,
        true,
        gdk::WindowExt::get_cursor(&window.get_window().unwrap()).as_ref(),
        None,
        None
    );
    if grab_status != gdk::GrabStatus::Success {
        panic!("Failed to acquire grab!");
    }
}
