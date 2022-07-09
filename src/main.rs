use x11::xlib;

use libc::{ c_uint };

mod wm;
mod events;
mod keys;
mod layouts;

// TODO: check if window size is less than ~10 when spawned
//       and remove from wm::WINDOWS. When running neofetch
//       from urxvt, it spawns a zombie-like window and doesnt
//       send a destroy request.

fn main() {
    let window_manager = wm::WindowManager::new();

    let keys = vec![keys::KeyPair::new((xlib::Mod4Mask | xlib::ShiftMask) as c_uint, "q", "quit"),
                    keys::KeyPair::new((xlib::Mod4Mask | xlib::ShiftMask) as c_uint, "c", "close-window"),
                    keys::KeyPair::new(xlib::Mod4Mask,                               "p", "spawn || dmenu_run"),
                    keys::KeyPair::new((xlib::Mod4Mask | xlib::ShiftMask) as c_uint, "Return", "spawn || urxvt")];

    window_manager.add_keys(keys);

    window_manager.run();

    window_manager.exit();
}
