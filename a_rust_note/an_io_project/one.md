# 一个I/O项目：构建一个命令行程序

- 目前所学之概括
- 更多标准库功能的探索

- 构建一个与文件和命令行输入输出交互的命令行工具

- 运行速度、安全性、单二进制文件输出、跨平台

- `grep`，全局搜索正则表达式并打印
  - 代码组织
  - vector和字符串
  - 错误处理
  - 合理使用trait和生命周期
  - 测试
  - 闭包、迭代器、trait对象


# 接收命令行参数

- 目标：`cargo run searchstring example-filename.txt`

## 读取参数值

- `std::env::args`，该函数返回一个传递给程序的命令行参数的迭代器iterator
- 迭代器生成一系列的值，可以在迭代器上调用collect方法将其转为一个集合

- `std::env:args`，只接受有效的Unicode字符
- `std""env::args_os`，可以接受无效的Unicode字符

- `collect`，是一个经常需要注明类型的函数，rust不能推断出你想要什么类型的集合

```rs
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
}
```

## 将值保存进变量

```rs
let query = &args[1];
let filename = &args[2];
```

## 读取文件

- 建立测试txt文件poem.txt

- `use std::fs;`

```rs
let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
```

- 下一步，由于main函数只负责简单的功能，所以独立封装
