use systems::game_engine;

mod models;
mod systems;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    game_engine::run()
}
