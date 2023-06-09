# 面向对象设计模式的实现

- 状态模式是一个面向对象设计模式
  - 该模式的关键在于一个值有某些内部状态，体现为一系列的状态对象，同时值得行为随着其内部状态而改变。
  - 状态对象共享功能：rust中使用结构体和trait而不是对象和继承

- 每一个状态对象负责其自身得行为，以及该状态何时应当转移至另一个状态
- 持有一个状态对象的值对于不同状态的行为以及何时状态转移毫不知情

- 博客：
  - 博文从空白得草案开始
  - 一旦草案完成，请求审核博文
  - 一旦博文过审，将被发表
  - 只有被发表的博文的内容会被打印，这样就不会意外打印出没有被审核的博文的文本

- 定义Post并新建一个草案状态的实例


- 将状态和行为编码为类型
- 