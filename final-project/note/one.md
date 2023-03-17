# 最后的项目：构建多线程web server

- 目的：返回hello的web server

- 计划：
  - 学习一些TCP和HTTP知识
  - 在套接字socket上监听TCP
  - 解析少量HTTP请求
  - 创建一个合适的HTTP响应
  - 通过线程池改善server的吞吐量

- 这里使用的方法并不是使用 Rust 构建 web server 最好的方法。crates.io 上有很多可用于生产环境的 crate，它们提供了比我们所要编写的更为完整的 web server 和线程池实现