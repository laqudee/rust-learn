# match

- 极为强大的控制流运算符，允许我们将一个值与一系列的模式进行比较，并根据匹配的模式执行相应的代码

- 模式可由字面值、变量、通配符和许多其他内容构成

- match的力量来源于模式的表现力以及编译器检查，他确保了所有可能的情况都得到处理。

```rs
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => {
      println!("Lucky penny");
      1
    },
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}
```

- 分支有两部分：一个模式和一些代码， =>将模式与将要运行的代码分开

- 绑定值的模式
  - 如何从枚举中提取值

```rs
#[derive(Debug)]
enum UsState {
  Alabama,
  Alaska,
  // --snip--
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
      println!("State quarter from {:?}!", state);
      25
    }
  }
}
```

- 匹配`Option<T>`

```rs
fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

- 匹配Some(T)

- 将 match 与枚举相结合在很多场景中都是有用的。
- 你会在 Rust 代码中看到很多这样的模式：match 一个枚举，绑定其中的值到一个变量，接着根据其值执行代码。
- 这在一开始有点复杂，不过一旦习惯了，你会希望所有语言都拥有它！这一直是用户的最爱

- 匹配是穷尽的
  - 分支必须覆盖了所有的可能性

- 通配符和`_`占位符

```rs
let dice_roll = 9;
match dice_roll {
  3 => add_fancy_hat(),
  7 => remove_fancy_hat(),
  other => move_player(other),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

// 使用占位符，而不是通配模式获取的值时
let dice_roll = 9;
match dice_roll {
  3 => add_fancy_hat(),
  7 => remove_fancy_hat(),
  _ => reroll(),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}

// 使用元组
let dice_roll = 9;
match dice_roll {
  3 => add_fancy_hat(),
  7 => remove_fancy_hat(),
  _ => (),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
```
