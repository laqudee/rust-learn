# 进一步认识Cargo 和 Crates.io

- Cargo一些更为高级的功能：
  - 使用发布配置来自定义构建
  - 将库发布到crates.io
  - 使用工作空间来组织更大的项目
  - 从crates.io安装二进制文件
  - 使用自定义的命令来扩展Cargo

# 采用发布配置自定义构建

- 运行`cargo build`时采用的dev配置
- 运行`cargo build --release`的release配置

- 构建输出中的dev和release表明编译器在使用不同的配置

