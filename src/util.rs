use x11::xlib;

use crate::wm;

pub fn focus_window(wm: &wm::WindowManager, w: u64) {
    unsafe {
        // first unfocus the original focused window
        unfocus_window(wm, wm::FOCUSED_WINDOW);

        wm::FOCUSED_WINDOW = w;

        xlib::XRaiseWindow(wm.display, wm::FOCUSED_WINDOW);

        // set window border and color:
        xlib::XSetWindowBorderWidth(wm.display, w, wm.config.border_width.try_into().unwrap());
        xlib::XSetWindowBorder(wm.display, w, wm.config.border_color_active);
    }
}

fn unfocus_window(wm: &wm::WindowManager, w: u64) {
    unsafe {
        // set window border and color:
        xlib::XSetWindowBorderWidth(wm.display, w, wm.config.border_width.try_into().unwrap());
        xlib::XSetWindowBorder(wm.display, w, wm.config.border_color_inactive);
    }
}
