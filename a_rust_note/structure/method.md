# 方法

- 与函数的不同：方法在结构体的上下文中被定义（或者是枚举或trait对象的上下文），它们的第一个参数总是self，它代表调用该方法的结构体实例。

- 方法定义

- `impl`块，是implementation的缩写
  - 这个 impl 块中的所有内容都将与 Rectangle 类型相关联。
  - 接着将 area 函数移动到 impl 大括号中，并将签名中的第一个（在这里也是唯一一个）参数和函数体中其他地方的对应参数改成 self

- 公有与私有

- 运算符到哪里去了？
  - 自动引用与解引用

- 更多参数

- 关联函数
  - 所有impl块中定义的函数被称为关联函数，因为它们与impl后面命名的类型相关。
  - 不是方法的关联函数经常被用来返回一个结构体新实例的构造函数。new，但new并不是关键字。

```rs
impl Rectangle {
  fn square(size: u32) -> Self {
    Self {
      width: size,
      height: size,
    }
  }
}

// 关键字 Self 在函数的返回类型中代指在 impl 关键字后出现的类型，在这里是 Rectangle

// 使用结构体名和::语法来调用整个关联函数
let sq = Rectangle::square(3)
```

- 模块

- 多个impl块
  - 每个结构体允许拥有多个impl块

```rs
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```


