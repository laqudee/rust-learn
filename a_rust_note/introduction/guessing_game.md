# 猜猜看程序的第一部分请求和处理用户输入，并检查输入是否符合预期的格式

- 首先，允许玩家输入猜测

- io输入/输出引用当前作用域
- io库来自于标准库，也被称为std

- 预导入内容，preclude

- fn main() {  程序入口点

- `let`
- 在rust中，变量默认是不可变的，一旦赋值，这个值就不再可以被修改
```rs
  let apples = 5; // 不可变
  let mut bananas = 5; // 可变
```

- String::new()

- `::new` 中 `::`语法表明new是String类型的一个关联函数。关联函数是针对类型实现的

- 静态方法

- `new`函数，它是创建类型实例的惯用函数名

- `use std::io;`从标准库中引入了输入/输出功能

```rs
  io::stdin()
    .read_line(&mut guess)

  // 可以写成
  std::io::stdin()
```

- `read_line()`的工作就是，无论用户在标准输入中键入什么内容，都加其追加到一个字符串中。这个字符串应该是可变的。

- `&`表示这个参数是一个饮用，它允许多处代码访问同一处数据，而无需再内存中多次拷贝。

- rust的一个主要优势就是安全而简单的操纵引用。

- Result类型将用来编码错误处理的信息， ok， err，Result实例包含expect方法

- `{}`是预留在特定位置的占位符，像小螃蟹一样可以夹住合适的值。
  - 也可以打印多个值

```rs
  let x = 5;
  let y = 10;

  println!("x = {} and y = {}", x, y)
```

- cargo update , 升级依赖的版本

# 生成一个秘密数字

- `use rand::Rng`

# 比较猜测的数字和秘密数字

- `use std::cmp::Ordering`

- rust 有一个静态强类型系统，也有类型推断

# 使用循环来允许多次猜测

- `loop`

- 正确退出循环
  - `break`

- 处理无效输入

```rs
  let guess: u32 = match guess.trim().parse() {
    OK(num) => num,
    Err(_) => continue,
  }
```

