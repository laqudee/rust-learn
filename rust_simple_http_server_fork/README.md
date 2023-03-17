# 一个Rust实现的简单的HTTPServer框架的Fork

## 已实现功能
- 解析http请求
  - post
  - get
- 响应http请求
  - 封装http协议常用状态码
- http请求路由处理逻辑
  - 路由功能
  - 多线处理
    - 1:1多线程（默认方式）
    - 线程池（在cargo中指定features = ["thread-pool"]）