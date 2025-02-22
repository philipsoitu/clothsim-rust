const X_COUNT: i32 = 2;
const Y_COUNT: i32 = 2;
const X_GAP: i32 = 10;
const Y_GAP: i32 = 10;

struct Node {
    curr_pos: (i32, i32),
    prev_pos: (i32, i32),
}

fn print_node(n: Node) {
    println!("[curr_pos]: x: {}, y: {}", n.curr_pos.0, n.curr_pos.1);
    println!("[prev_pos]: x: {}, y: {}", n.prev_pos.0, n.prev_pos.1);
}

fn main() {
    let mut points: Vec<Node> = Vec::new();

    for x in 0..X_COUNT {
        for y in 0..Y_COUNT {
            points.push(Node {
                curr_pos: (x * X_GAP, y * Y_GAP),
                prev_pos: (x * X_GAP, y * Y_GAP),
            });
        }
    }

    for point in points {
        print_node(point);
    }
}
