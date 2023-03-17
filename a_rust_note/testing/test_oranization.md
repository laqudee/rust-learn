# 测试的组织结构

- 单元测试 unit tests，倾向于更小更集中，在隔离环境中一次测试一个模块，或者是测试私有接口
- 集成测试 integration tests，对于代码而言完全是外部的。

## 单元测试

- 单元测试的代码共同存放在位于src目录下的相同文件中。
- 规范是在每个文件中创建包含测试函数的tests模块，并使用cfg(test)标注模块

- `#[cfg(test)]`注解告诉Rust只在执行cargo test时才编译和运行代码，在运行cargo build时不这样做

- cfg：configuration

- 集成测试位于另一个文件夹，所以不需要`#[cfg(test)]`注解

- 测试私有函数
  - rust的私有性规则允许测试私有函数

```rs
pub fn add_two(a: i32) -> i32 {
  internal_adder(a, 2)
}

fn internal_adder(a: i32, b:i32) -> i32 {
  a + b
}

#[cfg(test)]
mode tests {
  use super::*;

  #[test]
  fn internal() {
    assert_eq!(4, interal_adder(2, 2));
  }
}
```

- 在测试中，我们通过 use super::* 将 test 模块的父模块的所有项引入了作用域，接着测试调用了 internal_adder

## 集成测试

- 目的：测试库的多个部分能否一起正常工作

- 测试覆盖率

- tests目录

- `use adder`，因为每一个test目录中测试文件都是完全独立的crate，所以需要在每个文件中导入库

- `cargo test --test integration_test`，运行特定的集成测试

- 集成测试中的子模块
  - tests目录中的文件不能像src中的文件那样共享相同的行为

- 二进制crate的集成测试
  - `extern crate`
  -  Rust 二进制项目的结构明确采用 src/main.rs 调用 src/lib.rs 中的逻辑的方式？
  -  因为通过这种结构，集成测试 就可以 通过 extern crate 测试库 crate 中的主要功能了

