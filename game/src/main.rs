use game;

fn main() {
    if let Err(error) = game::initialize() {
        println!("Failed to initialize, error: {}", error);
        return;
    }
}
