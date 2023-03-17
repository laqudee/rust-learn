# 模式与模式匹配

- 模式是rust中特殊的语法，用来匹配类型中的结构，无论类型是简单还是复杂

- 结合使用模式和match表达式以及其他结构体可以提供更多对程序控制流的支配权

- 模式由如下一些内容组合而成
  - 字面值
  - 解构的数组、枚举、结构体或者元组
  - 变量
  - 通配符
  - 占位符

- 这些部分描述了我们要处理的苏剧的形状，接着可以用其匹配值来决定程序是否拥有正确的数据来运行特定部分的代码

# Refuability可反驳性：模式是否会匹配失效

- refutable，可反驳.对某些可能的值进行匹配会失败的模式
- irrefutable，不可反驳。能匹配任何传递的可能值的模式

- 函数参数、let语句、for循环只能接受不可反驳的模式

- if let 和 while let表达式被限制为只能接受可反驳模式

- match匹配分支必须使用可反驳模式，除了最后一个分支需要使用匹配任何剩余值得不可反驳模式

# 所有得模式语法

- 匹配字面值

```rs
let x = 1;

match x {
    1 => println!("One"),
    2 => println!("Two"),
}
```

- 匹配命名变量
  - 不可反驳模式

```rs
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
```

- 多个模式
  - `|`

```rs
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

- 通过 `..=`匹配值的范围

- 解构并分解值
  - 也可以使用模式来解构结构体、枚举、元组，以便于使用这些值的不同部分

