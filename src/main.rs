mod physics;
mod renderer;

use physics::Simulation;

#[macroquad::main("Cloth Sim")]
async fn main() {
    let mut simulation = Simulation::new();
    simulation.initialize(10, 10, 20.0, 20.0);
    renderer::run(simulation).await;
}
