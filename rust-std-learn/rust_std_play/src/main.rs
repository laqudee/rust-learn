mod modules;
mod primitive;

fn main() {
    primitive_play();
    modules_play();
}

fn primitive_play() {
    primitive::array::play();
    println!("++++++++++++++bool+++++++++++++++");
    primitive::bool::play();

    println!("++++++++++++++char++++++++++++++++");
    primitive::char::play();

    println!("+++++++++++pointer+++++++++++++++++");
    primitive::pointer::play();

    println!("++++++++++++reference+++++++++++++++");
    primitive::reference::play();

    println!("+++++++++++++slice++++++++++++++");
    primitive::slice::play();

    println!("++++++++++++str+++++++++++++++++++");
    primitive::str::play();

    println!("++++++++++++tuple+++++++++");
    primitive::tuple::play();

    println!("+++++++++++++unit++++++++++++++++++++");
    primitive::unit::play();
}

fn modules_play() {
    println!("++++++++++++++++++alloc++++++++++++++++++");
    modules::alloc::play();

    println!("+++++++++Any++++++++++++");
    modules::any::play();

    println!("++++++++++++++Boxed+++++++++++++");
    modules::boxed::play();

    println!("++++++++++++++++cell+++++++++++");
    modules::cell::play();

    println!("+++++++++++++collections+++++++++");
    modules::collections::play();

    println!("++++++++++++++hash++++++++++");
    modules::hash::play();

    println!("++++++++++++iter+++++++++++++");
    modules::iter::play();
}