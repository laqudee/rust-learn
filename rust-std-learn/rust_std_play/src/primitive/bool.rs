pub fn play() {
    let bool_false_then_some = false.then_some(0);
    let bool_true_then_some = true.then_some(3);
    println!(
        "true_then_some: {:?}; false_then_some: {:?}",
        bool_true_then_some, bool_false_then_some
    );

    let bool_false_then = false.then(|| 0);
    let bool_true_then = true.then(|| 2);
    println!(
        "true_then: {:?}; false_then: {:?}",
        bool_true_then, bool_false_then
    );
}
