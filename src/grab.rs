use gtk::prelude::*;
use gdk::WindowExt;

pub fn try_grab(seat: &gdk::Seat, window: &gtk::ApplicationWindow) -> Result<(), ()> {
    match seat.grab(
        &window.get_window().unwrap(),
        gdk::SeatCapabilities::ALL,
        true,
        window.get_window().unwrap().get_cursor().as_ref(),
        None,
        None
    ) {
        gdk::GrabStatus::Success => Ok(()),
        _ => Err(())
    }
}
