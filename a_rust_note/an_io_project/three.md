# 采用测试驱动开发完善库的功能

- TDD，测试驱动开发

- 搜索逻辑：
  1. 编写一个失败的测试，并运行它以确保它失败的原因是你所期望的
  2. 编写或修改足够的代码来使新的测试通过
  3. 重构刚刚增加或修改的代码，并确保测试仍然能够通过
  4. 从步骤1开始重复

## 编写失败测试

- 引入 `#[test]` 

- 显式生命周期'a
  - 生命周期参数指定哪个参数的生命周期与返回值的生命周期相关联

- 编写使测试通过的代码

- 使用lines方法遍历每一行
  - lines返回一个迭代器

- 用查询字符串搜索每一行
  - contains()

## 在run函数中使用search函数

```rs
for line in search(&config.query, &contents) {
    println!("{}", line);
}
```

# 处理环境变量

- 增加一个额外的功能来改进minigrep
  - 用户可以通过设置环境变量来设置搜索是否大小写敏感

## 编写一个大小写不敏感search函数的失败测试

- `use std::env`，处理环境变量的函数位于标准库的env模块中

- 一些程序允许对相同配置同时使用参数和环境变量

# 将错误信息输出到标准错误而不是标注输出

- 标准输出stdout

- 标准错误stderr

- 检查错误应该写入何处

- `cargo run > output.txt`

- 通过使用命令行参数、文件、环境变量和打印错误的eprintln!宏。
