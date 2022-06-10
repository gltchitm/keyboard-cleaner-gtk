use gtk::prelude::*;

pub fn try_grab(seat: &gdk::Seat, window: &gtk::ApplicationWindow) -> Result<(), ()> {
    match seat.grab(
        &window.window().unwrap(),
        gdk::SeatCapabilities::ALL,
        true,
        window.window().unwrap().cursor().as_ref(),
        None,
        None,
    ) {
        gdk::GrabStatus::Success => Ok(()),
        _ => Err(()),
    }
}
