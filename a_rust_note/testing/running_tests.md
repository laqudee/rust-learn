# 控制测试如何运行

- `cargo test`

- `cargo test --help`

- 并行或连续的运行测试
  - rust默认使用线程来并行运行。
  - `cargo test --test-threads=1`

- 显示函数输出
  - 默认情况下，当测试通过时，Rust 的测试库会截获打印到标准输出的所有内容。
  - 如果测试失败了，则会看到所有标准输出和其他错误信息
  - `cargo test -- --show-output`，告诉rust显示成功测试的输出

- 通过指定名字来运行部分测试
  - `cargo test one_hundred`

- 过滤运行多个测试
  - `cargo test add`
  - 可以指定部分测试的名称，任何名称匹配这个名称的测试都会被运行

- 忽略某些测试
  - `#[ignore]`

```rs
#[test]
fn it_works() {
  assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive() {
  // 需要一个小时的代码
}
```

- 只运行被忽略的测试`cargo test -- --ignored`
- 不管是否忽略都要全部运行`cargo test -- --include-ignored`
