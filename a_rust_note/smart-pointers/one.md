# 智能指针

- 指针pointer，是一个包含内存地址的变量的通用概念。这个地址引用，或“指向”一些其他数据

- & 引用，并借用了他们所指向的值

- 职能指针是一类数据结构，它们的表现类似指针，但是也拥有额外的元数据和功能

- 智能指针起源于C++，并存在于其他语言中。

- Rust标准库中不同的智能指针提供了多于引用的额外功能

- 引用计数，允许数据有多个所有者，引用计数智能指针记录总共有多少个所有者，并当没有任何所有者时负责清理数据。

- 普通引用和智能指针的一个额外的区别是引用是一类直接用数据的指针；相反，在大部分情况下，智能指针拥有他们指向的数据

- 例如，String和Vec[]

- 智能指针通常使用结构体实现，实现了Deref trait、Drop trait
- Deref trait 允许智能指针结构体实现表现的像引用一样，这样就可以编写既用于引用、又用于智能指针代码
- Drop trait允许自定义当指针离开作用域是运行的代码

- 智能指针是一个在Rust经常被使用的通过设计模式。

- `Box<T>`用于在堆上分配值
- `Rc<T>`一个引用计数类型，其数据可以有多个所有者
- `Ref<T>` 和`RefMut<T>`通过 `RefCell<T>` 访问。（ `RefCell<T>` 是一个在运行时而不是在编译时执行借用规则的类型）

- 内部可变性，这是不可变类型暴露出改变其内部值的 API

- 引用循环如何泄漏内存，以及如何避变

# 使用`Box<T>`指向堆上的数据


