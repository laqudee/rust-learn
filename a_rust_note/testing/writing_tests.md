# 如何编写测试

- 测试函数体通常执行如下操作：
  - 设置任何所需的数据或状态
  - 运行需要测试的代码
  - 断言其结果是我们所期望的

- test
- 宏
- should_panic

- test属性注解的函数，属性是关于rust代码片段的元数据

- `#[test]` cargo test

- `assert_eq!`宏 断言

- filtered out 过滤测试
- measured 针对性能测试

- Doc-tests adder 所有文档测试的结果

- panic!

- assert!，来检查结果，用来确保测试中一些条件为true
  - 如果值为true，assert!什么也不做
  - 如果值为false，assert!会调用panic!宏，导致测试失败

- assert_eq! 和assert_ne!宏来测试相等
  - 分别比较两个值是相等还是不等

- assert_eq!(expect, result)这个期望值与被测试代码的值的顺序并不重要
  - 底层使用==

```rs
  assert_eq!(4, add_two(2))
  //等同于
  assert_eq!(add_two(2), 4)
```

- assert_ne!宏在传递给他的两个值不相等时通过，而在相等时失败
  - 底层使用!=

- 这些被比较的值必须实现了PartialEql和Debug trait

- `#[derive(PartialEq, Debug)]` 注解

- 传递一个可选的失败信息参数
  - format!宏

- 使用should_panic检查panic
  - 检查代码是否按期望处理错误
  - should_panic测试结果可能会非常含糊不清，因为它只是告诉我们代码并没有产生panic
  - 可选属性expected参数

- 将`Result<T, E>`应用与测试

- 不能对使用`Result<T, E>`的测试使用`#[should_panic]`注解

- assert!(value.is_err())

