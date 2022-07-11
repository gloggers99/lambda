mod wm;
mod events;
mod keys;
mod layouts;
mod config;

// TODO: check if window has a name before considering it,
//       I am confident this will fix all of the ghost windows.

fn main() {
    let config = config::Config::new();
    let window_manager = wm::WindowManager::new(&config);

    window_manager.run();

    window_manager.exit();
}
