pub fn play() {
    let tuple = ("hello", 5, 'c');
    println!("{}-{}-{}", tuple.0, tuple.1, tuple.2);

    assert_eq!(tuple.0, "hello");

    let point = calculate_point();
    assert_eq!(point.0, 4);
    assert_eq!(point.1, 5);

    let (x, y) = calculate_point();
    println!("x: {}; y: {}", x, y);
}

fn calculate_point() -> (i32, i32) {
    (4, 5)
}
