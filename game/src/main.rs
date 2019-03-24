use game;

fn main() {
    if let Err(error) = game::initialize() {
        eprint!("Failed to initialize, error: {}", error);
    }
}
