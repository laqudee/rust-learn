# 构建单线程web server

- 涉及的主要协议
  - HTTP，超文本传输协议
  - TCP，传输控制协议

## 监听TCP连接

- web server第一件事就是能够监听TCP连接
- `std::net`

- `use std::net::TcpListener;`
  - TcpListener用于监听TCP连接
  - bind函数类似于new函数，返回一个新的TcpListener实例，返回`Result<T, E>`
  - incoming方法返回一个迭代器，提供一系列的流（更准确的说是TcpStream类型的流）

- 流stream代表一个客户端和服务端之间打开的连接
- 连接connection，代表客户端连接服务端、服务端生成响应以及服务端关闭连接的全部请求/响应过程

## 读取请求

- 实现读取来自浏览器请求的功能

- `use std::io::prelude`引入作用域来获取读写流所需的特定trait

- 在栈上声明一个buffer来存放读取到的数据，然后将缓冲区的字节转换为字符串并打印出来

- `String::from_utf8_loosy`函数获取一个`&[u8]`并产生一个String；lossy当遇到无效UTF-8序列使用特殊符号替代

## 仔细观察HTTP请求

```sh
Method Request-URI HTTP-Version CRLF
headers DRLF
message-body
```

- 从Host:开始的其余行是headers；

## 编写响应

- 实现在客户端请求的响应中发送数据的功能

```sh
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

- 定义了变量response来存放将要返回的成功响应的数据，接着调用as_bytes因为stream的write方法获取一个`&[u8]`并将这些字节发送给连接

## 返回真正的HTML

- 在开头增加了一行将标准库中的File引入作用域。

- 使用format!将文件内容加入到将要写入流的成功响应的body中

- 目前忽略了buffer中2的请求数据并无条件的发送了HTML文件的内容

- 检查请求并只对格式良好的请求/发送HTML文件

## 验证请求并有选择的进行响应

- 增加if else

- `b""`字节字符串语法将其转换为字节字符串，用于检查buffer是否以get中的字节开头

- 增加404.html

## 少量代码重构

- 目前if 和else中的代码有很多重复

