# 通过Deref trait将智能指针当作常规引用处理

- 实现Deref trait允许我们重载解引用运算符`*`
- 通过这种方式实现Deref trait的智能指针可以被当做常规引用来对待，可以编写操作引用的代码并用于智能指针

- 解引用运算

- Deref强制转换

## 通过解引用运算符追踪指针的值

- 常规引用是一个指针类型

## 像引用一样使用`Box<T>`

```rs
let x = 5;
let y1 = &x;
let y2 = Box::new(x);

assert_eq!(5, *y1); // 解引用
assert_eq!(5, *y2); // 解引用
```

- 主要不同的地方就是将y设置为一个指向x值拷贝的box实例而不是指向x值的引用

## 自定义职能指针
```rs
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```

- 通过实现Deref trait将某类型像引用一样处理

- 为了实现 trait，需要提供 trait 所需的方法实现。Deref trait，由标准库提供，要求实现名为 deref 的方法，其借用 self 并返回一个内部数据的引用

- `type Target = T`，语法定义了用于此trait的关联类型

- 关联类型是一个稍有不同的定义泛型参数的方式

- deref方法体中写入了&self.0，这样deref返回了我希望通过*运算符访问的值的引用

- rust 将*运算符替换成先调用deref方法，在进行普通解引用的操作，如此就不需要手动调用deref方法了

- deref 方法返回值的引用，以及 *(y.deref()) 括号外边的普通解引用仍为必须的原因在于所有权