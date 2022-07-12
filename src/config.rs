#[allow(unused_imports)]
use x11::xlib::{ Mod1Mask, Mod2Mask, Mod3Mask, Mod4Mask, ShiftMask };

use libc::{ c_uint, c_int };

use crate::keys::{ KeyPair };

// This file is where you will place your configuration options.
// Comments will guide you through the process, and it should be
// easy enough for someone who has never programmed to figure out.

// This structure is the configuration that will be passed to the
// window manager. Think of this as a template that tells you what
// your configuration can conatain. This is here so you can add
// aditional variables if you know what you are doing.
#[derive(Clone)]
pub struct Config {
    pub terminal: String,
    pub keys: Vec<KeyPair>,
    pub border_width: c_int,
    pub border_color_focused: u64,
    pub border_color_normal: u64
}

impl Config {
    pub fn new() -> Config {
        // Feel free to define variables here for later
        let terminal = String::from("alacritty");

        // This will initialize a new Config struct, place your config
        // options here.
        return Config {
            terminal: terminal,

            //                     Mod Key(s) (use |'s for multiple)  Key  Action
            keys: vec![KeyPair::new((Mod4Mask | ShiftMask) as c_uint, "q", "quit"),
                       KeyPair::new((Mod4Mask | ShiftMask) as c_uint, "c", "close-window"),
                       KeyPair::new(Mod4Mask,                         "p", "spawn || dmenu_run"),
                       KeyPair::new((Mod4Mask | ShiftMask) as c_uint, "Return", "spawn || alacritty"),
                       KeyPair::new(Mod4Mask,                         "r", "refresh")],

            border_width: 1,
            border_color_focused: 0xff0000,
            border_color_normal: 0xff0000
        };
    }
}
