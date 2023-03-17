# rust中的函数式语言功能：迭代器与闭包

- 函数式编程，通常包含将函数作为参数值或其他函数的返回值、将函数赋值给变量以供之后执行

- 闭包，一个可以存储在变量里的类似函数的结构
- 迭代器，一种处理元素序列的方式

# 闭包：可以捕获环境的匿名函数

- rust的闭包是可以保存在一个变量中或作为参数传递给其他函数的匿名函数

- 可以在一个地方创建闭包，然后在不同的上下文中执行闭包运算

- 闭包允许捕获被定义时所在作用域中的值

## 使用闭包创建行为的抽象

- 闭包的语法、类型推断、trait

```rs
use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
  println!("calculating slowly...");
  thread::sleep(Duration::from_secs(2));
  intensity
}

fn main() {
  let simulated_user_specified_value = 10;
  let simulated_random_number = 7;

  generate_workout(simulated_user_specified_value, simulated_random_number);
}
```

- 使用函数重构

- 重构使用闭包储存代码

```rs
let expensive_closure = |num| {
  println!("calculating slowlt...");
  thread::sleep(Duration::from_secs(2));
  num
}
```

- 闭包的定义以一对竖线|开始，在竖线中指定闭包的参数，这种语法与Ruby的闭包定义类似，
- 多余1个参数`|param1, param2|`
- 参数之后是存放包体的大括号，若闭包只有一行则大括号可以省略
- 闭包最后一行没有分号，like 函数，所以比包体最后一行的返回值作为调用闭包时的返回值

- 这个 let 语句意味着 expensive_closure 包含一个匿名函数的 定义，不是调用匿名函数的 返回值。

## 闭包类型推断和注解

- 闭包不要求像函数那样在参数和返回值上注明类型
- 函数中需要类型注解是因为他们是暴露给用户的显式接口的一部分
- 闭包并不用于这样暴露在外的接口：他们储存在变量中并被使用，不用命名他们或暴露给库的用户调用

- 闭包定义会为每个参数和返回值推断出一个具体类型

## 使用带有泛型和Fn trait的闭包

- 可以创建一个存放闭包和调用闭包结果的结构体，该结构体只会在需要结果时执行闭包，并会缓存结果值，这样余下的代码就不必再负责保存结果并可以复用该值
- 惰性求值（memoization） lazy evaluation

- Fn 系列 trait由标准库提供。所有的闭包都实现了trait Fn、 FnMut 或 FnOnce中的一个

- Fn trait bound

```rs
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}
```

- 函数也实现了这三个Fn trait。如果不需要捕获环境中的值，则可以使用实现了Fn trait的函数而不是闭包

- 不同于直接将闭包保存进一个变量，而是保存到一个Cacher实例中

## Cacher实现的限制

- 值缓存是一种更加广泛的实用行为，我们可能希望在代码中的其他闭包中使用他们。

- 问题：
  1. 一旦赋值，值不可改变，尝试修改Cacher存放一个哈希map而不是存放单独的值，通过key进行比较
  2. 接收的类型别限制，不够灵活 

## 闭包会捕获其环境

- 闭包拥有函数没有的功能：可以捕获其环境并访问其被定义的作用域的变量

```rs
fn main() {
  let x = 4;
  let equal_x = |z| x == z;

  let y = 4;

  assert!(equal_x(y));
}
```

- 当闭包从环境中捕获一个值，闭包会在闭包体中储存这个值以供使用。这会产生额外的内存开销

- 闭包可以通过三种方式捕获其环境：
  - FnOnce，只能调用一次
  - FnMut，获取可变的借用值所以可以改变其环境
  - Fn，从其环境获取不可变的借用值

- move关键字


