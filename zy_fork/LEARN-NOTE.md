# 学习模仿

> minimal and blazing-fast file server(最小和最快速的文件服务器)

## 报错处理

### 由于本机没有安装`visual code`，所以报`gcc.exe`找不到的错误

- 解决办法：
  - [https://www.msys2.org/](https://www.msys2.org/)
  - 安装`msys2-x86_64`

### cargo run，浏览器报错

- 刚开始没有查看`README.md`，直接启动，但是浏览器是无法打开`127.0.0.1:3000`，因为没有文件或文件件

## 正确cargo run dist

- `dist`是一个由`vue3`+`vant4`编写的`SPA`

- 这样是可以正常使用的，因为`dist`文件夹放在了根目录下

- 目前该`SPA`不涉及前后端接口交互

## 项目依赖简单释义：

- 本项目编译打包之后在`crates.io`上是`zy`

- clap, command line argument parser for rust(命令行参数解析)

- mime, Mime is now Media Type, technically, but Mime is more immediately underatandable, so the main type here is Mime.(简单来说就是多媒体类型枚举，各种传输参数的类型比如，application/json; bmp; css; html; png; svg; etc.)

- tokio, a runtime for writing reliable, asynchronous, slim
  - an event-driven, non-blocking I/O platform
    - multithreaded, work-stealing based task scheduler
    - asynchronous TCP and UDP socket
  - (运行时，包括多线程、工作流、网络连接、socket、操作系统支持等)

- tracing, application-level tracing(应用级跟踪)
  - a framework for instrumenting Rust programs to collect structured, event-based diaagnostic information(用于检测rust程序以收集结构化、基于事件的诊断信息的框架)

- actix-web, web framework for Rust
  - middlewares(Logger,Session, CORS, etc)
  - HTTP client
  - SSL support using OpenSSL or Rustls
  - Ststic asstes
  - Multipart streams
  - Transparent content compression
  - Client/server WebSockets
  - Keep-alive and slow request handling
  - full tokio compatibitling
  - request routing with optional macros
  - streaming and pipelining
  - HTTP/1.1 HTTP/2

- humantime, timestamp parsing/formatting is super-fast because format is basically fixed.
  - (时间格式化解析工具库)

- color-eyre,  an error report handler for panics and the `eyre` crate for colorful, consistent, and well formatted error reports for all kinds of errors.
  - (使得控制台错误报告更加清晰，易于读懂)

- actix-files, static file serving for Actix web

- tracing-subscriber, utilities for implementing and composing `tracing` subscribers.
  - (实现和组合tracing订阅者的工具)

## 进入main.rs

> 体验 `zy`在代码中的用法

```
cargo run dist
```