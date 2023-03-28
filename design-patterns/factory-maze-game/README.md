# Maze Game

> This example illustrates how to implement the Factory pattern using static dispatch(generics)

## Factory Pattern

- 工厂方法是一种创建型设计模式，解决了再不指定具体类的情况下创建产品对象的问题

- 工厂方法定义了一个方法，且必须使用该方法替代直接调用构造函数来创造对象（new操作符）的方式。子类可重写该方法来更改将被创建的对象所属类。