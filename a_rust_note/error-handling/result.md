# 用Result处理可恢复的错误

- 定义：
```rs
enum Result<T, E {
  Ok(T),
  Err(E)
}
```

```rs
use std::fs::File;

fn main() {
  let f = File::open("hello.text");

  let f = match f {
    Ok(file) => file,
    Err(error) => panic!("Problem opening the file: {:?}", error);
  };
}
```

- 匹配不同的错误

```rs
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}

// 使用闭包和unwrap_or_else
fn main() {
  let f = File::open("hello.txt").unwrap_or_else(|error| {
    if (error.kind() === ErrorKind::NotFound) {
      File::crate("hello.txt").unwrap_or_else(|error| {
        panic("Problem creating the file: {:?}", error);
      })
    } else {
      panic!("Problem opening the file: {:?}", error);
    }
  });
}

```

- closure

- 失败时panic的简写：unwrap和expect

```rs
use std::fs::File;

fn main() {
  let f = File::open("hello.txt").unwrap();

  let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

- 传播错误

```rs
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
  let f = File::open("hello.txt");

  let mut f = match f {
    Ok(file) => file,
    Err(e) => return Err(e),
  };

  let mut s = String::new();
  match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e),
  }
}

// 使用 ?

fn read_username_from_file() -> Result<String, io::Error> {
  let mut f = File::open("hello.txt")?;
  let mut s = String::new();
  f.read_to_string($mut s)?;
  Ok(s)
}

// 使用链式
fn read_username_from_file() -> Result<String, io::Error> {
  let mut s = String::new();

  File::open("hello.txt")?.read_to_string($mut s)?;

  Ok(s)
}

// 更简洁的写法，但是就没有展示所有这些错误的机会了
fn read_username_from_file() -> Result<String, io::Error> {
  fs::read_to_string("hello.txt")
}
```

- 传播错误的简写： `?`运算符


- 哪里可以使用?运算符
  - 只能被用于返回值与?作用域的值兼容的函数。
  - ?运算符被定义为从函数中提早返回一个值
  - 尝试在返回()的main函数中使用?的代码不能编译

- FromResidual类型

- 注意你可以在返回 Result 的函数中对 Result 使用 ? 运算符，可以在返回 Option 的函数中对 Option 使用 ? 运算符，但是不可以混合搭配。
- ? 运算符不会自动将 Result 转化为 Option，反之亦然；在这些情况下，可以使用类似 Result 的 ok 方法或者 Option 的 ok_or 方法来显式转换。

- main函数是特殊的因为它是可以执行程序的入口点和退出点，但main函数也可以返回`Result<T, E>`

```rs
use std::fs::File;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let f = File::open("hello.txt")?;

  Ok(())
}

// Box<dyn Error> 类型是一个trait对象，任何类型的错误
```

