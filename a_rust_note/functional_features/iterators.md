# 使用迭代器处理元素序列

- 迭代器模式允许开发者对一个序列的项进行某些处理。

- 迭代器负责遍历序列中的每一项和决定序列何时结束的逻辑。当使用迭代器时，我们无需重新实现这些逻辑

- 在rust中，迭代器是惰性的lazy，这意味着在调用方法使用迭代器之前他不会效果

```rs
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter();

for val in v1_iter {
  println!("Got: {}", val);
}
```

- for循环

- Iterator trait 和 next方法

```rs
pub trait Iterator {
  type Item;

  fn next(&mut self) -> Option<Self::Item>;
}
```

- `type Item`， `Self::Item`，定义了trait的关联类型

- `next`是Iterator实现者被要求定义的唯一方法，next 一次返回迭代器中的一个项，封装在 Some 中，当迭代器结束时，它返回 None

```rs
#[test]
fn iterator_demo() {
  let v1 = vec![1, 2, 3];

  let mut v1_iter = v1.iter();

  assert_eq!(v1_iter.next(), Some(&1));
  assert_eq!(v1_iter.next(), Some(&2));
  assert_eq!(v1_iter.next(), Some(&3));
  assert_eq!(v1_iter.next(), None);
}
```

- 迭代器可变 `iter_mut`

- 消费迭代器的方法
- 调用next方法的方法被称为消费适配器

```rs
#[test]
fn iterator_sum() {
  let v1 = vec![1, 2, 3];

  let v1_iter = v1.iter();

  let total: i32 = v1_iter.sum();

  assert_eq!(total, 6)

  // 调用 sum 之后不再允许使用 v1_iter 因为调用 sum 时它会获取迭代器的所有权
}
```

- 产生其他迭代器的方法
  - Iterator trait 中定义了另一类方法，被称为 迭代器适配器。
  - 允许我们将当前迭代器变为不同类型的迭代器。可以链式调用多个迭代器适配器

```rs
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, vec![2, 3, 4]);
```

- 迭代器适配器是惰性的

- 使用闭包获取环境

