use std::ptr;
use std::mem;

use x11::xlib;

use libc::{ c_int, c_uint };

use crate::events;
use crate::keys;
use crate::config;

pub static mut WINDOWS: Vec<xlib::Window> = vec![];
pub static mut KEYS: Vec<keys::KeyPair> = vec![];
pub static mut FOCUSED_WINDOW: u64 = 0;

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
    // save config here for later
    pub config: config::Config
}

impl WindowManager {
    pub fn new(config: &config::Config) -> WindowManager {
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

            let wm: WindowManager = WindowManager {
                display: display,
                root: root,
                config: config.clone()
            };

            Self::add_keys(&wm, wm.config.keys.clone());
            println!("Registered keys successfully.");

            // register clicks for focusing windows
            xlib::XGrabButton(display, 1, 0, xlib::XDefaultRootWindow(display), true as c_int,
                             (xlib::ButtonPressMask|xlib::ButtonReleaseMask|xlib::PointerMotionMask) as c_uint, xlib::GrabModeAsync, xlib::GrabModeAsync,
                             0, 0);

            xlib::XGrabButton(display, 3, 0, xlib::XDefaultRootWindow(display), true as c_int,
                             (xlib::ButtonPressMask|xlib::ButtonReleaseMask|xlib::PointerMotionMask) as c_uint, xlib::GrabModeAsync, xlib::GrabModeAsync,
                             0, 0);

            return wm;
        }
    }

    pub fn add_keys(wm: &WindowManager, keys: Vec<keys::KeyPair>) {
        unsafe {
            for key in keys {
                // register key on X server then push to `KEYS`
                xlib::XGrabKey(wm.display,
                               xlib::XKeysymToKeycode(wm.display,
                                                      xlib::XStringToKeysym(key.key.as_ptr())) as c_int,
                               key.modifier,
                               wm.root,
                               true as c_int,
                               xlib::GrabModeAsync,
                               xlib::GrabModeAsync);

                KEYS.push(key);
            }
        }
    }

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
                    xlib::ButtonPress => events::buttonpress_event(self, event),
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
