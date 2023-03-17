fn main() {
    let number = 3;
    
    if number < 5 {
        println!("is true");
    } else {
        println!("is false");
    }

    let_if();

    loop_start();

    loop_loop_start();

    while_start();

    for_start();

    for_rev_start();
}

fn let_if () {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}


fn loop_start () {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn loop_loop_start () {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // 语句将退出外层循环
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}")
}

fn while_start () {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!")
}

fn for_start () {
    let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("the value is: {}", a[index]);

    //     index += 1;
    // }

    for element in a {
        println!("the value is: {element}")
    }
}

fn for_rev_start () {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("OVER!!!")
}
