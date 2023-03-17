- cargo 是rust的构建系统和包管理器。

- `cargo --version`

- `cargo new project_name`

- `cargo new --vcs=git`

- TOML格式，这是cargo配置文件的格式

- 构建并运行cargo项目
  - `cargo build`, 创建一个可执行文件
  - `cargo run`, 同时编译并运行可执行文件
  - `cargo check`, 快速检查代码确保其可以编译，并不产生可执行文件。
  - `cargo build --release`, 优化编译项目，会在target/release而不是target/debug下生成可执行文件
