# 改进i/o项目

- 改进Config::new函数

- 改进search函数

- 使用迭代器并去除clone

- 使用Iterator trait代替索引

- 使用迭代器适配器来使代码更简明

# 性能对比：循环 vs 迭代器

- 迭代器作为一种高级抽象，被编译成了与手写的底层代码大体一致性能代码

- 迭代器时rust的零成本抽象之一，他意味着抽象并不会引入运行时开销

```rs
let buffer: &mut [i32];
let coefficients: [i64, 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
  let prediction = coefficients.iter()
                               .zip(&buffer[i - 12..i])
                               .map(|(&c, &s)| c * s as i64)
                               .sum::<i64>() >> qlp_shift;
  let delta = buffer[i];
  buffer[i] = prediction as i32  + delta;
}
```

- 请大胆使用迭代器和闭包吧

- 闭包和迭代器是rust👋函数式编程语言观念所启发的功能

- 闭包和迭代器的实现达到了不影响运行时性能的程度。这正是 Rust 竭力提供零成本抽象的目标的一部分
