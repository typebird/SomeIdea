mod generator;
mod interface;

fn main() {
    // let mut app = interface::AppInterface::new();

    // app.run();

    cursive::logger::init();
    // log::error!("Something serious probably happened!");
    // log::warn!("Or did it?");
    // log::debug!("Logger initialized.");
    // log::info!("Starting!");

    let mut siv = cursive::default();
    siv.add_layer(interface::main());
    siv.add_global_callback('q', interface::quit);
    siv.add_global_callback('~', cursive::Cursive::toggle_debug_console);
    // siv.add_global_callback('l', |_| log::trace!("Wooo"));
    siv.run();
}
