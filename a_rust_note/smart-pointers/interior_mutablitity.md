# `RefCell<T>`和内容可变性模式

- 内部可变性是Rust中的一个设计模式，它允许你即使在有不可变引用时也可以改变数据，这通常是借用规则所不允许的。

- 为了改变数据，该模式在数据结构中使用unsafe代码模糊rust通常的可变性和借用规则

- unsafe

## 通过`RefCell<T>在运行时检查借用规则

- `RefCell<T>`代表其数据的唯一的所有权

- 借用规则：
  - 任意给定时刻，只能拥有一个可变引用或任意数量的不可变引用之一
  - 引用必须总是有效的

- 对于引用和 `Box<T>`，借用规则的不可变性作用于编译时。对于 `RefCell<T>`，这些不可变性作用于 运行时。

- 在运行时检查借用规则的好处就是允许出现特定内存安全的场景，而它们在编译时检查中是不允许的

- `RefCell<T>`正是用于当确信代码遵守借用规则，而编译器不能理解和确定的时候

- `RefCell<T>`只能用于单线程场景

- 如下选择Box、Rc、RefCell的理由：
  - Rc允许相同数据有多个所有者；Box、RefCell有单一所有者。
  - Box允许在编译时执行不可变或可变借用检查；Rc仅允许在编译时执行不可变借用检查；RefCell允许在运行时执行不可变或可变借用检查。
  - RefCell允许在运行时执行可变借用检查，所以我们可以在即便RefCell自身是不可变的情况下修改其内部的值

- 在不可变值内部改变值就是内部可变性模式

## 内部可变性：不可变值得可变借用

```rs
// error
let x = 6;
let y = &mut x; 
```

- 特定情况下，令一个值在其方法内部能够修改自身，而在其他代码中视为不可变，是很有用的。

- `RefCell<T>`是一个获得内部可变性的方法

## 内部可变性的用例：mock对象

- 测试替身是一个通用编程概念，代表一个在测试中替代某个类型的类型。

- `borrow()`

- `borrow_mut()`

## `RefCell<T>`在运行时记录借用
- &， borrow()，返回`Ref<T>`
- &mut，borrow_mut()，返回`RefMut<T>`

- `RefCell<T>` 记录当前有多少个活动的 `Ref<T>` 和 `RefMut<T>` 智能指针。
- 每次调用 borrow，`RefCell<T>` 将活动的不可变借用计数加一。当 `Ref<T>` 值离开作用域时，不可变借用计数减一。
- 就像编译时借用规则一样，`RefCell<T>` 在任何时候只允许有多个不可变借用或一个可变借用

```rs
// error: 在任何时候只允许有多个不可变借用或一个可变借用
impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        let mut one_borrow = self.sent_messages.borrow_mut();
        let mut two_borrow = self.sent_messages.borrow_mut();

        one_borrow.push(String::from(message));
        two_borrow.push(String::from(message));
    }
}
```

- 编译时不会报错，但运行时会报错

- 在运行时捕获借用错误而不是编译时意味着将会在开发过程的后期才会发现错误，甚至有可能发布到生产环境才发现；还会因为在运行时而不是编译时记录借用而导致少量的运行时性能惩罚

## 结合`Rc<T>`和`RefCell<T>`来拥有多个可变数据所有者

- 

