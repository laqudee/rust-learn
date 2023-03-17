# 重构改进模块性和错误处理

 - 当前程序存在的问题：
   - main函数中的任务太多，最后分离一个函数一个功能
   - 将配置变量组织进入一个结构体
   - 文件打开处理过于简单
   - 错误处理不够清晰

## 二进制项目的关注分离

- 步骤：
  - 将程序拆分为mian.rs和lib.rs，将逻辑放到lib.rs中

- main.rs的职责被限制为：
  - 使用参数值调用命令行解习逻辑
  - 设置任何其他的配置
  - 调用lib.rs中的run函数
  - 如果run返回错误，则处理这个错误

- 提取参数解析器

- 组合配置值

- 使用clone的权衡取舍

- 创建一个Config的构造函数

- 修复错误处理
  - 改善错误信息
  - 从new中返回Result而不是调用panic
  - Config::new()调用并处理错误

- `unwrap_or_else`它定义于标准库的`Result<T, E>`上。
  - 用来进行一些自定义的非panic!的错误处理

- 闭包closure，也就是一个我们定义的作为参数传递给`unwrap_or_else`的匿名函数。
  - 现在你需要理解的是 unwrap_or_else 会将 Err 的内部值，也就是示例 12-9 中增加的 not enough arguments 静态字符串的情况，传递给闭包中位于两道竖线间的参数 err。闭包中的代码在其运行时可以使用这个 err 值。

## 从main中提取逻辑

- fn run() {}
  - run函数包含了main中从读取文件开始的剩余所有逻辑
  - run函数获取一个Config实例作为参数

- 从run函数中返回错误

- trait对象`Box<dyn Error>`，意味着函数会返回实现了Error trait的类型，不过无需指定具体将会返回的值的类型。
  - dyn：dynamic动态的的缩写


## 处理main中run返回的错误

```rs
if let Err(e) = run(config) {
    println!("Application error: {}", e);

    process::exit(1);
}
```

## 将代码拆分到库crate
- lib.rs 专注于逻辑
- main.rs 专注于业务

