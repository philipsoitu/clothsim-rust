use crate::physics::Simulation;
use macroquad::prelude::*;

pub async fn run(mut simulation: Simulation) {
    loop {
        clear_background(WHITE);

        simulation.update();

        for node in &simulation.nodes {
            draw_circle(node.curr_pos.0, node.curr_pos.1, 10.0, RED);
        }

        next_frame().await;
    }
}
