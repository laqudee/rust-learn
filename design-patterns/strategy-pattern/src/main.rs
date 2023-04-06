// 策略模式

mod conceptual;
mod functional;

use conceptual::{Navigator, WalkingStrategy, PublicTransportStrategy};

use functional::it_work;

fn main() {
    let navigator = Navigator::new(WalkingStrategy);
    navigator.route("Home", "Club");
    navigator.route("Club", "Work");

    let navigator = Navigator::new(PublicTransportStrategy);
    navigator.route("Home", "Club");
    navigator.route("Club", "Work");

    println!("functional way: ------------------");
    it_work();
}
