use x11::xlib;

use libc::{ c_int };

use crate::wm;

// TODO: abstract most of layout to make it easier for users to make custom layouts
pub fn tile(wm: &wm::WindowManager) {
    unsafe {
        if wm::WINDOWS.len() > 0 {
            let master_window: u64 = *wm::WINDOWS.first().unwrap();
            let mut stack_windows: Vec<u64> = vec![];
            let mut stack_windows_step: i32 = 0; // y value of last tiled stack window

            let screen = xlib::XDefaultScreenOfDisplay(wm.display);

            let screen_width: u32 = xlib::XWidthOfScreen(screen).try_into().unwrap();
            let screen_height: u32 = xlib::XHeightOfScreen(screen).try_into().unwrap();

            for w in &wm::WINDOWS {
                if w != &master_window {
                    // get attributes and check if the window is viewable
                    let mut attr: xlib::XWindowAttributes = std::mem::zeroed();
                    let ret = xlib::XGetWindowAttributes(wm.display, *w, &mut attr);

                    if ret != 0
                        && attr.map_state != xlib::IsUnviewable
                        && attr.override_redirect == false as c_int {
                        stack_windows.push(*w);
                    }
                }
            }

            if wm::WINDOWS.len() == 1 {
                xlib::XMoveResizeWindow(wm.display, master_window,
                                        0, 0,
                                        screen_width - (wm.config.border_width * 2) as u32,
                                        screen_height - (wm.config.border_width * 2) as u32);
            } else {
                xlib::XMoveResizeWindow(wm.display, master_window,
                                        0, 0,
                                        (screen_width / 2) - (wm.config.border_width * 2) as u32,
                                        screen_height - (wm.config.border_width * 2) as u32);

                for w in &stack_windows {
                    xlib::XMoveResizeWindow(wm.display, *w,
                                           (screen_width / 2) as i32, stack_windows_step,
                                           screen_width / 2 - (wm.config.border_width * 2) as u32,
                                           (screen_height / stack_windows.len() as u32) - (wm.config.border_width * 2) as u32);

                    stack_windows_step += screen_height as i32 / stack_windows.len() as i32;
                }
            }
        }
    }
}
