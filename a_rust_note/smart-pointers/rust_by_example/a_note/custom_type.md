# 自定义类型

- Rust自定义数据类型主要是通过
  - struct：定义一个结构体structure
  - enum：定义一个枚举类型enumeration

- 常量constant，可以const 和 static 关键字创建

# 结构体

- 元组结构体tuple struct 事实上就是具名元组
- 经典的C语言风格结构体
- 单元结构体，不带字段，在泛型中很有用

# 枚举

- enum关键字允许创建一个从数个不同取值中选其一的枚举类型enumeration

- 任何一个在struct中合法的取值在enum中也合法

- 类型别名
  - 若使用类型别名，则可以通过其别名引用每个枚举变量
  - type Alias = EnumName
  - Self别名
