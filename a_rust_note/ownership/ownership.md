# 所有权

- rust的核心功能之一就是所有权

- 所有程序都必须管理其运行时使用计算机内存的方式。

- 类似JavaScript、Java，在程序运行时有规律地寻找不再使用的内存
- 类似C、C++，就需要手动分配和释放内存

- Rust选择通过所有权系统管理内存，编译器在编译时会根据一系列的规则进行检查，不符合规则程序无法编译通过。

- 栈stack与堆heap
  - rust极为关注栈和堆，值是位于栈上还是堆上影响着语言的行为

- 栈：先进后出，大小固定

- 堆：缺乏组织的，指针，在堆上分配内存

- 栈比堆快

- 当你的代码调用一个函数时，传递给函数的值（包括可能指向堆上数据的指针）和函数的局部变量被压入栈中。当函数结束时，这些值被移出栈。

- 所有权的主要目的就是为了管理堆数据

# 所有权规则
1. rust中的每个值都有一个所有者owner
2. 值在任何时刻有且只有一个所有者
3. 当所有者（变量）离开作用域，这个值将被丢弃


- 变量作用域
  - scope作用域，是一项在程序中有效的范围

- string类型

# slice类型

- slice是一类引用，所以没有所有权

```rs
fn first_world(s: &String) -> usize {
  let bytes = s.as_bytes();
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return i;
    }
  }
  s.len()
}
```

- 字符串slice是String中一部分值得引用

```rs
  let s = String::from("hello world");
  let hello = &s[0..5];
  let world = &s[6..11];
```

- `[starting_index..ending_index]`，starting_index是第一个位置，ending_index是最后一个2位置得后一个值

- slice得数据结构存储了slice得开始位置和长度，长度就是ending_index - starting_index的值

- 对于Rust的..range语法，如果想要从索引0开始，可以不写两点之前的值

```rs
  let s = String::from("hello");
  let slice = &s[0..2];
  let slice2 = &s[..2];

  // 如果slice包含String的最后一个字节，也可以舍弃尾部的数字
  let len = s.len();
  let slice3 = &s[3..len];
  let slice4 = &s[3..];

  let slice5 = &s[0..len];
  let slice6 = &s[..];
```

- 字符串slice range的索引必须位于的UTF-8字符边界内

```rs
  fn first_world(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
      if item === b' ' {
        return &s[0..i];
      }
    }

    &s[..]
  }
```

- 字符串字面值就是slice
  - `let s = "hello world";`，s的类型是`&str`：它是一个指向二进制程序特定位置的slice。所以字符串字面值是不可变的; &str是一个不可变引用

```rs
  fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
      if item === b' ' {
        return &s[0..i];
      }
    }

    &s[..]
  }

  fn main () {
    let my_string = String::from("hello world");

    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    let word = first_word(&my_string);

    // `first_word` 适用于字符串字面值，整体或全部
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // 因为字符串字面值已经 **是** 字符串 slice 了，
    // 这也是适用的，无需 slice 语法！
    let word = first_word(my_string_literal);
  }
```

- 其他类型的slice，例如 数组

- 所有权、借用和slice这些概念让rust程序在编译时确保内存安全

