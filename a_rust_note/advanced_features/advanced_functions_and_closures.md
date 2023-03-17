# 高级函数和闭包

- 函数指针以及返回值闭包

- 函数指针
  - 通过函数指针允许我们使用函数作为另一个函数的参数
  - 函数的类型是fn，以免和Fn闭包trait相混淆
  - fn被称为函数指针
  - 指定参数为函数指针的语法类似于闭包

```rs
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is : {}", answer);
}
```

- 不同于闭包，fn是一个类型而不是trait，所以直接指定fn作为参数而不是2声明一个带有Fn作为trait bound的泛型参数

- 函数指针实现了所有三个闭包trait（Fn、FnMut、FnOnce），所以总是可以在调用期望闭包的函数时传递函数指针作为参数

- 倾向于编写使用泛型和闭包 trait 的函数，这样它就能接受函数或闭包作为参数

- 一个只期望接受 fn 而不接受闭包的情况的例子是与不存在闭包的外部代码交互时：C 语言的函数可以接受函数作为参数，但 C 语言没有闭包

- 完全限定语法

- 另一个实用的模式暴露了元组结构体和元组结构体枚举成员的实现细节

```rs
enum Status {
    Value(u32),
    Stop,
}

let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
```

- 返回闭包
  - 闭包表现为trait，这意味着不能直接返回闭包
  - 对于大部分需要返回trait的情况，可以使用实现了期望返回的trait的具体类型来替代函数的返回值会

```rs
// error

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x+1)
}
```