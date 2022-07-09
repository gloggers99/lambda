use std::ffi::CString;
use std::process;

use x11::xlib;

use libc::{ c_uint };

use crate::wm;

pub fn action_parser(action: &String, wm: &wm::WindowManager, event: xlib::XKeyEvent) {
    let action_split: Vec<&str> = action.split(" ").collect();

    match action_split[0] {
        "quit" => {
            wm.exit();
        },
        "close-window" => {
            unsafe {
                for w in &wm::WINDOWS {
                    if w == &event.subwindow {
                        xlib::XDestroyWindow(wm.display, *w);
                    }
                }
            }
        },
        "spawn" => {
             let action_cmd: Vec<&str> = action.split(" || ").collect();
             let _ = process::Command::new(action_cmd[1]).stdin(process::Stdio::null())
                                                         .spawn()
                                                         .expect("Failed to spawn process");
        },
        "refresh" => {
            println!("refreshing");
            wm.refresh();
        },
        _ => {
            println!("Invalid action!");
        }
    }
}

pub struct KeyPair {
    pub modifier: c_uint,
    pub key: CString,
    pub action: String
}

impl KeyPair {
    pub fn new(modifier: c_uint, key: &str, action: &str) -> KeyPair {
        KeyPair {
            modifier: modifier,
            key: CString::new(key).unwrap(),
            action: action.to_owned()
        }
    }
}
