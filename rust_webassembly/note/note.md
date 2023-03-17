# 编译Rust 为 WebAssembly

> 将Rust代码编译为wasm并在一个现存的web应用中使用之。

## Rust 和 WebAssembl用例
- Rust 和 WebAssembly两大主要用例：
  - 构建完整应用——整个Web应用都基于Rust开发！yew框架
  - 构建应用的组成部分——在现存的JavaScript前端中使用Rust（rust团队正专注于第二种用例）

- 本例，将使用Rust的npm包构建工具wasm-pack来构建一个npm包。
  - 这个包只包含WebAssembly和JavaScript代码，以便于包的用户无需安装Rust就能使用使用者甚至不需要知道包含WebAssembly

