use systems::runtime;

mod models;
mod systems;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    runtime::run()
}

// todo: start game screen and selecting start game speed
// todo: up game speed based on game score
// todo: restart game from end gamescreen
// todo: error handlings (removing unwraps)
