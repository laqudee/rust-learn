fn main() {
    println!("Hello, world!");
    another_function(234);
    print_label(5, 'h');

    let y = {
        let x = 3;
        x +1
    };
    println!("The value of y is: {y}");

    let main_five = five();
    println!("The value of main_five is: {main_five}");

    let main_x = plus_one(5);
    println!("The value of main_x is {main_x}")
}

fn another_function (x: i32) {
    println!("The value of x is: {x}");
}

fn print_label(value: i32, unit_label: char) {
    println!("The  is: {value}{unit_label}")
}

// 具有返回值的函数
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
