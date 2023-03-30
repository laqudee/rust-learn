#[derive(Clone)]
struct Circle {
    pub x: u32,
    pub y: u32,
    pub radius: u32,
}

fn main() {
    let circle1 = Circle {
        x: 10,
        y: 15,
        radius: 10,
    };

    let circle2 = circle1.clone();

    println!("Circle 1: {}, {}, {}", circle1.x, circle1.y, circle1.radius);
    println!("Circle 2: {}, {}, {}", circle2.x, circle2.y, circle2.radius);
}
