# 生命周期确保引用有效

- rust中的每一个引用都有其生命周期，也就是确保引用保持有效的作用域。

- 所以rust需要我们使用泛型生命周期参数来注明他们的关系，这样就能确保运行时实际使用的引用绝对是有效的。

- 生命周期注解甚至不是一个大部分语言都有的概念。

- 生命周期避免了悬垂引用

```rs
{
  let r;
  {
    let x = 5;
    r = &x;
  }

  println!("r: {}", r);
}

// error:r的引用值在尝试使用之前就离开了作用域
```

- 作用域越大，存活越久

- rust编译器有一个借用检查器，它比较作用域来确保所有的借用都是有效的。

- 只能引用生命周期比自己大的值，才不会产生编译错误

- 函数中的泛型生命周期

- 生命周期注解语法
- `'a`

```rs
&i32

&'a i32
&'a mut i32
```

- 指定生命周期参数的正确方式依赖函数实现的具体功能

- 生命周期语法是用于将函数的多个参数与其返回值的生命周期进行关联的。
  - 一旦他们形成了某种关联，rust就有了足够的信息来允许内存安全的操作并阻止产生悬垂指针或者是违反内存安全的行为。

- 结构体定义中的生命周期注解
  - 我们将定义包含引用的结构体，不过这需要为结构体定义中的每一个引用添加生命周期注解

- 生命周期省略

- 输入生命周期

- 输出生命周期

- 方法定义中的生命周期注解
```rs
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

```

- 静态生命周期
  - 'static，其生命周期能够存活于整个程序期间

- 泛型类型参数、trait、trait bounds、泛型生命周期类型
