mod wm;
mod events;
mod keys;
mod layouts;
mod config;

// TODO: for the most part, the ghost window bug is fixed,
//       pavucontrol spawns an extra blank window, but it
//       for some reason has a name (bruh who came up with that)
//       so that remains an issue.

// TODO: make window border change based on focus

fn main() {
    let config = config::Config::new();
    let window_manager = wm::WindowManager::new(&config);

    window_manager.run();

    window_manager.exit();
}
