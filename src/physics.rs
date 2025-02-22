pub struct Node {
    pub curr_pos: (f32, f32),
    pub prev_pos: (f32, f32),
}

impl Node {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            curr_pos: (x, y),
            prev_pos: (x, y),
        }
    }

    pub fn update(&mut self) {
        self.prev_pos = self.curr_pos;
        self.curr_pos.0 += 1.0;
        self.curr_pos.1 += 1.0;
    }
}

pub struct Simulation {
    pub nodes: Vec<Node>,
}

impl Simulation {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn initialize(&mut self, x_count: i32, y_count: i32, x_gap: f32, y_gap: f32) {
        for x in 0..x_count {
            for y in 0..y_count {
                println!("{} {}", x, y);
                self.nodes.push(Node {
                    curr_pos: ((x as f32) * x_gap, (y as f32) * y_gap),
                    prev_pos: ((x as f32) * x_gap, (y as f32) * y_gap),
                });
            }
        }
    }

    pub fn update(&mut self) {
        for node in &mut self.nodes {
            node.update();
        }
    }
}
