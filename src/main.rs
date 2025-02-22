mod physics;
mod renderer;

use physics::Simulation;

#[macroquad::main("Cloth Sim")]
async fn main() {
    const X_COUNT: i32 = 21;
    const Y_COUNT: i32 = 21;
    let mut simulation = Simulation::new(X_COUNT * Y_COUNT);
    simulation.initialize(21, 21, 20.0, 20.0, 100.0, 100.0);
    renderer::run(simulation).await;
}
