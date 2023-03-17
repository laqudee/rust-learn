- 可恢复的错误，`Result<T, E>`
- 不可恢复的错误，`panic!宏`

# 用panic!处理不可恢复的错误

- 对应panic时的栈展开或终止

- 直接终止
```toml
[profile.release]
panic = 'abort'
```

- backtrace 回溯
  - 是一个执行到目前位置所有被调用的函数的列表

# 要不要panic

- 返回Result是定义可能会失败的函数的一个好的默认选择

- 在一些类似示例、原型代码（prototype code）和测试中， panic 比返回 Result 更为合适，不过他们并不常见

- 错误处理指导原则：
  - 在当有可能会导致有害状态的情况下建议使用 panic!
  - 然而当错误预期会出现时，返回 Result 仍要比调用 panic! 更为合适。
