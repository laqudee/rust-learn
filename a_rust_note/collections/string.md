# String

- rust的核心语言中只有一种字符串类型，slice str

- String类型由标准库提供

```rs
// 从new函数创建字符串
let mut s = String::new();

// to_string()创建字符串
let data = "initial contents";
let s = data.to_string();
let s1 = "initial contents".to_string()

// from函数
fn main() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

```

- 更新字符串

- `+` `fomat!`宏拼接String值

```rs
// push_str push附加字符串
let mut s = String::from("foo");
s.push_str("bar");

// push_str方法采用字符串slice，并不需要获取参数的所有权
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {}", s2) // right code

// push方法被定义为获取一个单独的字符作为参数，并附加到String中
let mut s = String::from("lo");
s.push('l');

// +运算符或format!拼接
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // s1被移动了，不能继续使用

// +使用了add函数
fn add(self, s:&str) -> String {}
```

- Deref强制转换，将&s2变为了`&s2[..]`

```rs
fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    // 过于繁琐，使用format!宏替代
    let s = format!("{}-{}-{}", s1, s2, s3);
}
```

- 索引字符串
  - Rust的字符串不支持索引

```rs
let s1 = String::from("hello");
let h = s1[0];
// error
```

- String是一个Vec<u8>的封装

- 字节、标量值、字形簇

- 索引操作预期总是需要常数时间O(1)，但是对于String不可能保证这样的性能，因为Rust必须从开头到索引位置遍历来确定有多少有效字符

- 字符串slice

```rs
let hello = "Здравствуйте";

let s = &hello[0..4];
```

- 遍历字符串的方法
  - 操作字符串每一部分的最好方法是明确需要字符还是字节，
  - 对于Unicode标量值使用chars方法

```rs
#![allow(unused)]
fn main() {
  for c in "नमस्ते".chars() {
      println!("{}", c);
  }

  // bytes()返回每一个原始字节
  for b in "नमस्ते".bytes() {
    println!("{}", b);
  }
}

```

