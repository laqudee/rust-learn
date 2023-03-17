# 高级类型

- newtype与类型一样有用

- 类型别名

- `!`类型

- 动态大小类型

## 为了类型安全和抽象而使用newtype模式

- 静态的确保某值不被混淆，和用来表示一个值的单元

- 抽象掉一些类型的实现细节，例如封装类型可以暴露出与直接使用其内部私有类型时所不同的公有API，以便于限制其功能

- newtype还可以隐藏内部的泛型类型

- newtype模式是一种隐藏实现细节的封装的轻量级方法

## 类型别名用来创建类型同义词

- 类型别名，使用type关键字来给予现有类型另一个名字
```rs
type Kilometers = i32;

let x: i32 = 5;
let y: Kilometers = 5;

println!("x + y = {}", x + y);
```

- 类型别名的主要用途是减少重复

```rs
Box<dyn Fn() + Send + 'static>

type Thunk = Box<dyn Fn() + Send + 'static>

let f: Thunk = Box::new(|| println!("hi"));

fn takes_long_type(f: Thunk) {

}

fn returns_loong_type() -> Thunk {

}
```

- 类型别名也经常与`Result<T, E>`结合用来减少重复

```rs
use std::fmt;
use std::io::Error;

type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}
```

## 从不返回的never type

- `!`类型，empty type，没有值，或者称之为never type
  - 作用：在函数从不返回的时候充当返回值

```rs
fn bar() -> ! {

}
```

- 从不返回的函数被称为发散函数

- 描述!的行为的正式方式是never type可以强转为任何其他类型

- 允许match的分支以continue结束是因为continue并不真正返回一个值；相反它把控制权交回上层循环，所以在Err的情况，事实上并未对guess赋值

- never type的另一个用途就是panic!

- panic! 是 ! 类型，所以整个 match 表达式的结果是 T 类型

- loop也有!类型

```rs
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}

print!("forever")
loop {
    print!(and ever)
}
```

## 动态大小类型和Sized trait

- DST或unsized types

- str是一个DST

```rs
// error
let s1: str = "Hello there!";
let s2: str = "How's it going?"
```

- &str有两个值：str的地址和其长度

- 动态大小类型的黄金法则：必须将动态大小类型的值置于某种指针之后

- sized trait

- rust隐式的为每一个泛型函数增加了Sized bound

```rs
fn generic<T>(t: T) {

}

// 实际上
fn generic<T: Sized>(t: T) {

}
```

- 泛型函数默认只能用于编译时已知大小的类型
  
```rs
fn generic<T: ?sized>(t: &T) {

}
```

- ?Sized上的trait bound意味着T可能是也可能不是sized 同时这个注解会覆盖泛型类型必须在编译时拥有固定大小的默认规则。
- 这种意义的 ?Trait 语法只能用于 Sized ，而不能用于任何其他 trait。

- 另外注意我们将 t 参数的类型从 T 变为了 &T：因为其类型可能不是 Sized 的，所以需要将其置于某种指针之后。在这个例子中选择了引用。
