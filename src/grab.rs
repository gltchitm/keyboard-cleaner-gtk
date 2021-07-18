use super::gtk::prelude::*;

pub fn try_grab(seat: &gdk::Seat, window: &gtk::ApplicationWindow) -> Result<(), ()> {
    match seat.grab(
        &window.get_window().unwrap(),
        gdk::SeatCapabilities::ALL,
        true,
        gdk::WindowExt::get_cursor(&window.get_window().unwrap()).as_ref(),
        None,
        None
    ) {
        gdk::GrabStatus::Success => Ok(()),
        _ => Err(())
    }
}
