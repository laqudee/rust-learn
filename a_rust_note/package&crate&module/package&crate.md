- 一个包可以包含多个二进制crate项和一个可选的crate库。

- 外部依赖项

- cargo workspaces

- 公有部分与私有部分

- 作用域：代码所在的嵌套上下文有一组定义为in scope的名称

- 模块系统：
  - 包packages：cargo的一个功能，他允许你构建、测试和分享crate
  - crates：一个模块的树形结构，它形成了库或二进制项目
  - modules和use：允许你控制作用域和路径的私有性
  - path：一个命名例如结构体、函数或模块等项的方式

# package and crate

- crate是rust在编译时最小的代码单位。
- 如果你用rustc而不是cargo来编译一个文件，编译器还是会将那个文件认作一个crate。
- crate可以包含模块，模块可以定义在其他2文件。然后和crate一起编译

- crate有两种形式：
  - 二进制项，可以被编译为可执行程序，如一个命令行程序或者一个服务器。都有一个main函数
  - 库，并没有main函数，不会编译为可执行程序，它们提供一些诸如函数之类的东西，使其他项目也能使用这些东西

- 一般而言，crate和其他编程语言里的library概念一致

- crate root，rust编译器的起始点

- 包：提供一系列功能的一个或者多个crate，一个包会包含一个cargo.toml

- cargo就是一个包含构建rust代码的二进制项的包


