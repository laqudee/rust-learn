# vector

- vector允许我们在一个单独的数据结构中存储多于一个的值，它在内存中彼此相邻

- vector只储存相同类型的值

```rs
  // 调用Vec::new()
  let v: Vec<i32> = Vec::new();

  // 使用vec!宏
  let v = vec![1, 2, 3];

  // 更新vector
  let mut v = Vec::new();
  v.push(5);
  v.push(6);

  // 读取vector的元素
  let v = vec![1,2,3,4,5];
  let third: &i32 = &v[2];
  println!("The third element {}", third);

  match v.get(2) {
    Some(third) => println!("the third element is {}", third),
    None => println!("not found"),
  }
```

- `Vec<T>`是用泛型实现的

- `vec!`宏

- vector在其离开作用域时会被释放

- 访问vector中值的方式：索引语法、get方法

- `[]`访问一下不存在的元素会造成panic，但是get不会，只会返回none

- 在vector的结尾增加新元素时，在没有足够空间将所有元素依次相邻存放的情况下，可能要求分配新内存并将老的元素拷贝到新的空间中。
  - 这时，第一个元素的引用就指向了被释放的内存。借用规则阻止程序陷入这种状况。

- 遍历

```rs
let v = vec![100, 32, 57];
for i in &v {
  println!("{}", i);
}

let mut v = vec![100, 32, 57];
for i in &mut v {
  *i += 50;
}
```

- 使用枚举来储存多种类型
```rs
  enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }

  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
  ]
```

- rust在编译时就必须准确知道vector中类型的原因在于它需要知道储存每个元素到底需要多少内存

- pop()


