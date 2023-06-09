# 引用循环与内存泄漏

- rust 的内存安全性保证使其难以意外地制造永远也不会被清理的内存（内存泄漏），但也不是不可能。

- rust并不能保证完全地避免内存泄漏，这意味着内存泄漏在rust被认为是内存安全的。

- 这一点可以通过 `Rc<T>` 和 `RefCell<T>` 看出：创建引用循环的可能性是存在的。这会造成内存泄漏，因为每一项的引用计数永远也到不了 0，其值也永远不会被丢弃

## 制造引用循环
- List枚举
- tail方法

- 如果在更为复杂的程序中并在循环中分配了很多内存并占有很长时间，这个程序会使用多余它所需的内存，并压垮系统

- 如果你有包含`Rc<T>`的`RefCell<T>`值或类似的嵌套结合了内部可变性和引用计数的类型，小心确保你没有形成一个引用循环

- 创建引用循环是一个程序上的逻辑bug，应该使用自动化测试、代码评审和其他软件开发最佳实践来使其最小化

- 另一个解决方案就是重新组织数据结构，使得一部分引用拥有所有权而另一部分没有。

- 换句话说，循环将由一些拥有所有权的关系和一些无所有权的关系组成，只有所有权关系才能影响值是否可以被丢弃

## 避免引用循环：将`Rc<T>`变为`Weak<T>`

- Rc::clone(&source)
- strong_count，必须为0才能被清理

- 通过调用Rc::downgrade并传递`Rc<T>`实例的引用来创建期值的弱引用（Weak reference）

- `Weak<T>`
- weak_count，无须为0就可以被清理

- 强引用代表如何共享`Rc<T>`实例的所有权，但弱引用并不属于所有权关系。他们不会造成引用循环，因为任何弱引用的循环会在其相关的强引用引用计数为0时被打断。

- upgrade()，返回`Option<Rc<T>>`
  - 如果`Rc<T>`的值还没有被丢弃，则结果是Some
  - 如果`Rc<T>`的值已被丢弃，则结果是None

## 创建树形数据结构：带有子节点的Node
- 让我们创建一个用于存放其拥有所有权的 i32 值和其子节点引用的 Node

```rs
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}
```

- 希望能够Node拥有其子节点，同时也希望通过变量来共享所有权，以便于可以直接访问树中的每一个Node，为此`Vec<T>`的项的类型被定义为`Rc<Node>`。
- 还希望能修改其他节点的子节点，所以children中`Vec<Rc<Node>>`被放进了`RefCell<Vec<Rc<Node>>>`

## 增加从子到父的引用
- 为了使子节点知道其父节点，需要在Node结构体定义中增加一个parent字段，Weak类型的用武之地

- parent使用 `Weak<T>`类型而不是`Rc<T>`

## 总结
- 如何使用智能指针来做出不同于 Rust 常规引用默认所提供的保证与取舍

- `Box<T>` you一个已知大小并指向分配在堆上的数据
- `Rc<T>`记录了堆上数据的引用数量以便于可以拥有多个所有者
- `RefCell<T>`和其内部可变性提供了一个可以用于当需要不可变类型但是需要改变其内部值能力的类型，并在运行时而不是编译时检查借用规则

- 智能指针功能的 trait Deref 和 Drop

- `Weak<T>`

