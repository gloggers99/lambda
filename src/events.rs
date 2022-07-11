use x11::xlib;

use crate::wm;
use crate::keys;
use crate::layouts;

// executed when a window is created
pub fn createnotify_event(wm: &wm::WindowManager, event: xlib::XEvent) {
    unsafe {
        let e: xlib::XCreateWindowEvent = From::from(event);


        xlib::XSetWindowBorderWidth(wm.display, e.window, wm.config.border_width.try_into().unwrap());
        xlib::XSetWindowBorder(wm.display, e.window, wm.config.border_color_normal);

        wm::WINDOWS.push(e.window);

        layouts::tile(wm);
    }
}

pub fn destroynotify_event(wm: &wm::WindowManager, event: xlib::XEvent) {
    unsafe {
        let e: xlib::XDestroyWindowEvent = From::from(event);

        let mut index: usize = 0;

        for w in &wm::WINDOWS {
            if *w == e.window {
                wm::WINDOWS.remove(index);
                layouts::tile(wm);
                return;
            }

            index += 1;
        }

        panic!("Failed to remove window from {:?}", wm::WINDOWS);
    }
}

pub fn keypress_event(wm: &wm::WindowManager, event: xlib::XEvent) {
    unsafe {
        let e: xlib::XKeyEvent = From::from(event);

        for key in &wm::KEYS {
            if xlib::XStringToKeysym(key.key.as_ptr()) == xlib::XKeycodeToKeysym(wm.display, e.keycode.try_into().unwrap(), 0) {
                keys::action_parser(&key.action, wm, e);
            }
        }
    }
}

//pub fn buttonpress_event() {}

//pub fn configurenotify_event() {}

//pub fn configurerequest_event() {}

//pub fn mapnotify() {}

pub fn maprequest(wm: &wm::WindowManager, event: xlib::XEvent) {
    unsafe {
        let e: xlib::XMapRequestEvent = From::from(event);

        xlib::XMapWindow(wm.display, e.window);
    }
}
