# Cargo 工作空间

- workspaces，管理多个相关的协同开发的包

- 创建工作空间
  - 共享同样的cargo.lock和输出目录的包。
  - 有多种组织工作空间的方式

- 实例项目：
  - 工作空间有一个二进制项目和两个库
  - 二进制项目提供主要功能、并依赖另两个库

- 工作空间在顶级目录有一个target目录
  - 子目录没有target目录

- 通过共享target目录，工作空间可以避免其他crate多余的重复构建

- 在工作空间中创建第二个包

- cargo 并不假定工作空间中的Crates会相互依赖，所以需要明确表明工作空间中crate的依赖关系

- 为了在顶层add目录运行二进制crate，可以通过 -p 参数和包名称来运行cargo run 指定工作空间我们希望使用的包

- 在工作空间中依赖外部包
  - 还需要注意的是工作空间只在根目录有一个Cargo.lock，而不是在每一个crate目录都有Cargo.lock。
  - 这确保了所有的crate都使用完全相同版本的依赖

- 也可以选择运行工作空间中特定 crate 的测试，通过在根目录使用 -p 参数并指定希望测试的 crate 名称

# 使用cargo install 从crates.io安装二进制文件

- 所有来自cargo install的二进制文件都安装到rust安装根目录的bin文件夹中

```rs
cargo install ripgrep
```

# Cargo 自定义扩展命令

- Cargo 的设计使得开发者可以通过新的子命令来对 Cargo 进行扩展，而无需修改 Cargo 本身。

- 如果 $PATH 中有类似 cargo-something 的二进制文件，就可以通过 cargo something 来像 Cargo 子命令一样运行它。

- 像这样的自定义命令也可以运行 cargo --list 来展示出来。

- 能够通过 cargo install 向 Cargo 安装扩展并可以如内建 Cargo 工具那样运行他们是 Cargo 设计上的一个非常方便的优点
