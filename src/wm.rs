use std::ptr;
use std::mem;

use x11::xlib;

use libc::{ c_int, c_uint };

use crate::events;
use crate::keys;

pub static mut WINDOWS: Vec<xlib::Window> = vec![];
pub static mut KEYS: Vec<keys::KeyPair> = vec![];

// error handler for X11
pub unsafe extern "C" fn x_error_handler(_display: *mut xlib::Display,
    e: *mut xlib::XErrorEvent) -> i32 {
    if (*e).error_code == xlib::BadAccess {
        panic!("Another WM is already running!");
    } else {
        eprintln!("An X11 Error was triggered: {}", (*e).error_code);
    }

    return 0;
}

pub struct WindowManager {
    pub display: *mut xlib::Display,
    pub root: xlib::Window,
}

impl WindowManager {
    pub fn new() -> WindowManager {
        unsafe {
            xlib::XSetErrorHandler(std::option::Option::Some(x_error_handler));

            let display = xlib::XOpenDisplay(ptr::null_mut());
            let root = xlib::XDefaultRootWindow(display);

            xlib::XSelectInput(
                display,
                root,
                xlib::SubstructureRedirectMask | xlib::SubstructureNotifyMask
            );

            println!("Connected to X server successfully.");

            return WindowManager {
                display: display,
                root: root
            };
        }
    }

    pub fn add_keys(&self, keys: Vec<keys::KeyPair>) {
        unsafe {
            for key in keys {
                // register key on X server then push to `KEYS`
                xlib::XGrabKey(self.display,
                               xlib::XKeysymToKeycode(self.display,
                                                      xlib::XStringToKeysym(key.key.as_ptr())) as c_int,
                               key.modifier,
                               self.root,
                               true as c_int,
                               xlib::GrabModeAsync,
                               xlib::GrabModeAsync);

                KEYS.push(key);
            }
        }
    }

    // INCOMPLETE
    pub fn frame_window(&self, w: u64) {
        unsafe {

            // take these variables in a config file later
            let border_width: c_uint = 3;
            let border_color = 0xff0000;
            let bg_color = 0x0000ff;

            let mut attrs: xlib::XWindowAttributes = std::mem::zeroed();

            let ret = xlib::XGetWindowAttributes(self.display, w, &mut attrs);

            if ret == 0 {
                panic!("XGetWindowAttributes failed.");
            }
        }
    }

    // TODO: we need to frame the window before we add it
    //       to wm::WINDOWS
    pub fn refresh(&self) {
        unsafe {
            // clear wm::WINDOWS
            WINDOWS.clear();

            // add existing windows to wm::WINDOWS
            let mut root_return = 0;
            let mut parent_return = 0;
            let mut children_return = ptr::null_mut();
            let mut nchildren_return = 0;

            let ret = xlib::XQueryTree(self.display,
                                       self.root,
                                       &mut root_return,
                                       &mut parent_return,
                                       &mut children_return,
                                       &mut nchildren_return);

            if ret == 0 {
                panic!("XQueryTree failed.");
            }

            for i in 0..nchildren_return {
                let offset = children_return.offset(i.try_into().unwrap());
                let child = *offset;

                WINDOWS.push(child);
            }
        }
    }

    pub fn run(&self) {
        unsafe {
            xlib::XSync(self.display, false as c_int);

            self.refresh();

            loop {
                let mut event: xlib::XEvent = mem::zeroed();
                xlib::XNextEvent(self.display, &mut event);

                match event.get_type() {
                    xlib::CreateNotify => events::createnotify_event(self, event),
                    xlib::DestroyNotify => events::destroynotify_event(self, event),
                    xlib::KeyPress => events::keypress_event(self, event),
                    //xlib::ButtonPress => println!("Button Pressed Event"),
                    //xlib::ConfigureNotify => println!("Configure Notify"),
                    //xlib::ConfigureRequest => println!("Configure Request"),
                    //xlib::MapNotify => println!("Map Notify"),
                    xlib::MapRequest => events::maprequest(self, event),
                    _ => {}
                }
            }
        }
    }

    pub fn exit(&self) {
        unsafe {
            xlib::XCloseDisplay(self.display);
        }

        println!("Disconnected from X server successfully.");

        std::process::exit(0);
    }
}
