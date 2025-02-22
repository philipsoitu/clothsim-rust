use std::cell::RefCell;
use std::rc::Rc;

const GRAVITY: (f32, f32) = (0.0, 90.8);
const DIST: f32 = 10.0;

pub struct Node {
    pub curr_pos: (f32, f32),
    pub prev_pos: (f32, f32),
    pub immovable: bool,
}

impl Node {
    pub fn new(x: f32, y: f32, immovable: bool) -> Self {
        Self {
            curr_pos: (x, y),
            prev_pos: (x, y),
            immovable,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if !self.immovable {
            let new_x = 2.0 * self.curr_pos.0 - self.prev_pos.0 + GRAVITY.0 * dt * dt;
            let new_y = 2.0 * self.curr_pos.1 - self.prev_pos.1 + GRAVITY.1 * dt * dt;
            self.prev_pos = self.curr_pos;
            self.curr_pos = (new_x, new_y);
        }
    }
}

pub struct Link {
    pub a: Rc<RefCell<Node>>,
    pub b: Rc<RefCell<Node>>,
    pub dist: f32,
}

impl Link {
    pub fn new(a: Rc<RefCell<Node>>, b: Rc<RefCell<Node>>, dist: f32) -> Self {
        Self { a, b, dist }
    }
    pub fn update(&mut self, dt: f32) {}
}

pub struct Simulation {
    pub nodes: Vec<Rc<RefCell<Node>>>,
    pub links: Vec<Link>,
}

impl Simulation {
    pub fn new(node_count: i32) -> Self {
        Self {
            nodes: Vec::with_capacity(node_count as usize),
            links: Vec::new(),
        }
    }

    pub fn initialize(
        &mut self,
        x_count: i32,
        y_count: i32,
        x_gap: f32,
        y_gap: f32,
        x_pad: f32,
        y_pad: f32,
    ) {
        for x in 0..x_count {
            for y in 0..y_count {
                self.nodes.push(Rc::new(RefCell::new(Node::new(
                    (x as f32) * x_gap + x_pad,
                    (y as f32) * y_gap + y_pad,
                    y == 0 && x % 5 == 0,
                ))));

                if y != 0 {
                    self.links.push(Link {
                        a: Rc::clone(&self.nodes[self.nodes.len() - 1]),
                        b: Rc::clone(&self.nodes[self.nodes.len() - 2]),
                        dist: DIST,
                    })
                }

                if x != 0 {
                    self.links.push(Link {
                        a: Rc::clone(&self.nodes[self.nodes.len() - 1]),
                        b: Rc::clone(&self.nodes[self.nodes.len() - 1 - (x_count as usize)]),
                        dist: DIST,
                    })
                }
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        for node in &mut self.nodes {
            node.borrow_mut().update(dt);
        }
    }
}
