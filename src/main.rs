mod generator;
mod interface;

fn main() {
    let mut app = interface::AppInterface::new();

    app.run();
}
