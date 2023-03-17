# 顾及不同类型值得trait对象

- 将创建一个图形用户接口工具的例子，它通过遍历项目并调用每一个项目的draw方法来将其绘制到屏幕中

- gui
  - Button
  - TextField
  - Image
  - SelectBox

- Component的类，该类也有一个draw方法，其他类比如Button从Component中派生并继承draw方法

- 覆盖

- Rust没有继承

## 定义通用行为的trait

- Draw trait
- 定义一个存放trait 对象的vector
- trait对象指向一个实现了我们指定trait的类型的实例，以及一个用于再运行时查找该类型的trait方法的表

- 使用trait对象代替泛型或者具体类型

- 再结构体或枚举中，结构体字段中的数据和impl块中的行为是分开的，不同于其他语言中将数据和行为组合进一个称为对象的概念中

- trait对象将数据和行为两者结合，从某种意义上讲则其更类似其他语言中的对象、

- 不能向trait对象增加数据，trait 对象具体的作用是允许对通用行为进行抽象

- 一个带draw方法的trait Draw
  
```rs
pub trait Draw {
    fn draw(&self);
}
```

- 定义一个存放叫components的vector的结构体Screen
  
```rs
pub trait Draw {
    fn draw(&self);
}
```

## 实现trait
- 现在来增加一些实现了Draw trait的类型

## trait 对象执行行为动态分发

- 当对泛型使用trait bound时编译器所执行的单生态化处理：编译器为每一个被泛型类型参数代替的具体类型生成了函数和方法的非泛型实现

- 静态分发，发生于编译器在编译时
- 动态分发，编译器编译时无法知晓调用了什么方法

## trait对象需要类型安全

- 只有对象安全的trait可以实现为特征对象

- 对象安全规则：
  - 返回值不是Self
  - 没有泛型类型的参数

```rs
// 这是不安全的
pub trait Clone {
  fn clone(&self) -> Self;
}
```