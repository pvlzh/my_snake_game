use systems::runtime;

mod models;
mod systems;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    runtime::run()
}
