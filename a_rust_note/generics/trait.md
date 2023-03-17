# trait：定义共同行为

- trait告诉rust编译器某个特定类型拥有可能与其他类型共享的功能

-  trait bounds 

- trait类似其他语言中的接口interfaces的功能，虽然有些不同

- 一个类型的行为由其可供调用的方法构成。
- 如果可以对不同类型调用相同的方法的话，这些类型就可以共享相同的行为了
- trait定义是一种将方法签名组合起来的方法，目的是定义一个实现某些目的所必须的新闻给的集合。

- trait 关键字来声明一个trait，后面是trait的名字。
- 我们声明trait为pub以便于依赖这个crate的crate也可以使用这个trait。

- 在方法签名后跟分号，而不是在大括号中提供其实现。接着每一个实现这个trait的类型都需要提供其自定义行为的方法体

- trait体中可以有多个方法：一行一个方法签名且都以分号结尾

- 为类型实现trait

- 在类型上实现trait类似于实现与trait无关的方法。区别在于impl关键字之后，要提供需要实现trait的名称，接着是for和需要实现trait类型的名称。

- trait必须和类型一起引入作用域以便于使用额外的trait方法。

```rs
use aggregator::{Summary, Tweet};

fn main() {
  let tweet: Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people."),
    reply: false,
    retweet: false,
  };

  println!("1 new tweet: {}", tweet.summarize());
  // 1 new tweet: horse_ebooks: of course, as you probably already know, people
}
```

- 相干性
- 孤儿规则

- 默认实现
  - 有时为trait中的某些或全部方法提供默认的行为，而不是在每个类型的每个实现中都定义自己的行为是很有用的。
  - 这样当为某个特定类型实现trait时，可以选择保留或重载每个方法的默认行为

- 重载一个默认实现的语法与实现没有默认实现的trait方法的语法一样

- 默认实现允许调用相同trait中的其他方法，哪怕这些方法没有默认实现

```rs
pub trait Summary {
  fn summarize_author (&self) -> String;

  fn summarize(&self) -> String {
    format!("(Read more from {}...)", self.summarize_author())
  }
}

impl Summary for Tweet {
  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }
}

let tweet = Tweet {
  username: String::from("horse_ebooks"),
  content: String::from("of course, as you probably already know, people"),
  reply: false,
  retweet: false,
};

println!("1 new tweet: {}", tweet.summarize());
```

- 无法从相同方法的重载实现中调用默认方法，但可以在默认方法中调用重载方法

- trait作为参数

```rs
pub fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}
```

- 对于item参数，我们指定了impl关键字和trait名称，而不是具体的类型。
  - 该参数支持任何实现了指定trait的类型。

- Trait Bound语法
  - 是一种较长形式的语法糖

```rs
pub fn notify<T: Summary>(item: &T) {
  println!("Breaking news! {}", item.summarize())
}

// trait bound 与泛型参数声明在一起，位于尖括号中的冒号后面
```

- impl Trait 适合用短小的例子

- trait bound适用于更复杂的场景

```rs
pub fn notify(item1: &impl Summary, item2: &impl Summary) {}

// trait bound
pub fn notify<T: Summary>(item1: &T, item2: &T) {}
```

- 通过 + 指定多个trait bound

```rs
pub fn notify(item: &(impl Summary + Display)) {}

// trait bound
pub fn notify<T: Summary + Display>(item: &T)
```

- 通过where简化trait bound

```rs
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// where
fn some_function<T, U>(t: &T, u: &U) -> i32
  where T: Display +  Clone,
        U: Clone + Debug
  {}
```

- 返回实现了trait的类型
  - 在返回值中使用impl Trait语法，来返回实现了某个trait的类型

```rs
fn return_summraizable() -> impl Summary {
  Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people",),
    reply: false,
    retweet: false,
  }
}

// 通过使用impl Summary作为返回值类型
```

- 返回一个只是指定了需要实现的trait的类型的能力在闭包和迭代器场景十分的有用。

- 使用trait bounds来修复largest函数

- 生命周期

- 使用trait bound有条件地实现方法

```rs
use std::fmt::Display;

struct Pair<T> {
  x: T,
  y: T,
}

impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    Self { x, f }
  }
}

impl<T: Display + PartialOrd> Pair<T> {
  fn cmp_display(&self) {
    if self.x >= self.y {
      println!("The largest member is x =  {}", self.x);
    } else {
      println!("The largest member is y = {}", self.y);
    }
  }
}
```

```rs
impl<T: Display> ToString for T {
  // --snip--
}

let s = 3.to_string();
```


