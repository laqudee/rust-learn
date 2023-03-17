- 函数的签名问题

- 改为元组

- 元组帮助我们增加了一些结构性，但是元组没有给出元素的名称

- 改为结构体

- 我们希望借用结构体而不是获取它的所有权，这样main()就可以保持rect1的所有权并继续使用它。所以这就是为什么在函数签名和调用的地方会有 &


- 通过派生trait增加实用功能

- 基本类型都默认实现了Display，但是对于结构体，println!打印出来的格式是不明确的。

```rs
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  println!("rect1 is {:?}", rect1);
}
```

- `dbg! 宏`

```rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
```
