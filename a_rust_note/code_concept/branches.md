# 控制流

- if 表达式

- 代码中的条件必须是bool值。如果不是布尔值，将得到一个错误
- rust不香JavaScript那样，rust不会自动将非布尔值转为布尔值

- let 语句中使用if

```rs
  fn let_if () {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}")
  }
```

- if 与 else的分支的值的类型要相容，否则会报错

- 使用循环重复执行
  - loop
  - 一个循环执行体中待代码直到结尾并紧接着回到开头继续执行。

- loop
- while
- for

- 循环标签：在多个循环之间消除歧义
  - loop
  - break
  - continue

- while 条件循环

- for遍历集合
