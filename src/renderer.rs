use crate::physics::Link;
use crate::physics::Node;
use crate::physics::Simulation;
use macroquad::prelude::*;
use macroquad::time::get_frame_time;

const RADIUS: f32 = 2.0;
const THICKNESS: f32 = 1.0;

fn draw_nodes(nodes: &Vec<Node>) {
    for n in nodes {
        draw_circle_lines(n.curr_pos.0, n.curr_pos.1, RADIUS, THICKNESS, BLACK);
    }
}

fn draw_links(links: &Vec<Link>, nodes: &Vec<Node>) {
    for link in links {
        let la = nodes[link.a].curr_pos;
        let lb = nodes[link.b].curr_pos;

        draw_line(la.0, la.1, lb.0, lb.1, THICKNESS, BLACK);
    }
}

pub async fn run(mut simulation: Simulation) {
    loop {
        clear_background(WHITE);

        let dt = get_frame_time();
        simulation.update(dt);

        draw_nodes(&simulation.nodes);
        draw_links(&simulation.links, &simulation.nodes);

        next_frame().await;
    }
}
