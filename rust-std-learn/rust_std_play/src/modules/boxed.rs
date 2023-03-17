pub fn play() {
    let val: u8 = 5;
    let boxed = Box::new(val);
    println!("boxed: {:?}", boxed);

    let boxed: Box<u8> = Box::new(5);
    let val = *boxed;
    println!("val: {}", val);

    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{list:?}");
}

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}
