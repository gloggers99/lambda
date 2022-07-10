mod wm;
mod events;
mod keys;
mod layouts;
mod config;

fn main() {
    let config = config::Config::new();
    let window_manager = wm::WindowManager::new(&config);

    // manually add keys
    //window_manager.add_keys(config.keys);

    window_manager.run();

    window_manager.exit();
}
