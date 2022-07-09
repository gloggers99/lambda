use x11::xlib;

use crate::wm;

pub fn tile(wm: &wm::WindowManager) {
    unsafe {
        if wm::WINDOWS.len() > 0 {
            let master_window: u64 = *wm::WINDOWS.first().unwrap();
            let mut stack_windows: Vec<u64> = vec![];
            let mut stack_windows_step: i32 = 0; // y value of last tiled stack window

            let screen = xlib::XDefaultScreenOfDisplay(wm.display);

            let screen_width: i32 = xlib::XWidthOfScreen(screen);
            let screen_height: i32 = xlib::XHeightOfScreen(screen);

            for w in &wm::WINDOWS {
                if w != &master_window {
                    stack_windows.push(*w);
                }
            }

            if wm::WINDOWS.len() == 1 {
                xlib::XMoveResizeWindow(wm.display, master_window,
                                        0, 0,
                                        xlib::XWidthOfScreen(screen).try_into().unwrap(),
                                        xlib::XHeightOfScreen(screen).try_into().unwrap());
            } else {
                xlib::XMoveResizeWindow(wm.display, master_window,
                                        0, 0,
                                        (screen_width / 2).try_into().unwrap(),
                                        screen_height.try_into().unwrap());

                for w in &stack_windows {
                    xlib::XMoveResizeWindow(wm.display, *w,
                                           screen_width / 2, stack_windows_step,
                                           (screen_width / 2).try_into().unwrap(),
                                           (screen_height / stack_windows.len() as i32).try_into().unwrap());

                    stack_windows_step += screen_height / stack_windows.len() as i32;
                }
            }
        }
    }
}
