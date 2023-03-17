# 将一个包发布到crates.io

- 编写有用的文档注释

- 文档注释类型，文档注释，会生成HTML文档

- `///`

- `cargo doc --open`，会构建当前crate文档（同时还有所有crate依赖对的文档）的HTML并在浏览器中打开

- 常用（文档注释）部分
  - Panics：这个函数可能会panic的场景
  - Errors：如果这个函数返回Result。次部分描述可能会出现何种错误一级什么情况下会造成这些错误，这有助于调用者编写代码来采用不同的方式处理不同的错误
  - Safety：如果这个函数使用 unsafe 代码，这一部分应该会涉及到期望函数调用者的确保unsafe块中代码正常工作的不变条件

- 文档注释作为测试
  - 在文档注释中增加示例代码块是一个清楚的表明如何使用库的方法
  - cargo test 也会像测试那样运行文档中的示例代码

- 注释包含项的结构
  - 另一种文档注释`//!`，这是包含注释的项，而不是位于注释之后的项增加文档
  - 通常位于crate根文件`src/lib.rs`或模块的根文件为crate或模块整体提供文档

## 使用 pub use 导出合适的公有API

- mod 将代码组织进模块
- pub 将项变成公有
- use 将项引入作用域

```rs
use my_crate::some_module::another_module::UsefulType;

// 用户更喜欢
use my_crate::UsefulType;
```

- pub use 重导出项 re-export 来使公有结构不同于私有结构
  - 重导出获取位于一个位置的公有项并将其公开到另一个位置，好像它定义在新位置一样

```rs
use art::kinds::PrimaryColor;
use art::utils::mix;

// 改为
use art::mix;
use art::PrimaryColor;

fn main() {
  let red = PrimaryColor::Red;
  let yellow = PrimaryColor::Yellow;
  mix(red, yellow);
}
```

## 创建Crate.io账号

- `cargo login cio*****NPsGFip******leqsfvHiVZ***x`
  - 这个命令会通知Cargo你的API token 并将其储存在本地的`~/.cargo/credentials`文件

## 发布新crate之前

- `cargo publish`

- crate名先到先得，一个名称一旦被使用，其他人就不可以再使用

- 该crate用途的描述、用户可能在何种条款下使用该crate的license，需要在toml中引入这些信息

## 发布到crates.io
- 一旦发布将不允许删除

## 使用cargo yank 从crates.io撤回版本

```sh
cargo yank --vers 1.0.1

# 也可以撤销撤回操作，并允许项目可以再次开始依赖某个版本
cargo yank --vers 1.0.1 --undo
```
