# 关键字

- 函数
- 变量
- 参数
- 结构体
- 模型
- crate
- 常量
- 宏
- 静态值
- 熟悉
- 类型
- trait
- 生命周期

- as，强制类型转换，消除特定包含项的trait的歧义，或者对use语句中的项重命名
- async 返回一个Future而不是阻塞当前线程
- await，暂停执行直到Future的结果就绪
- break，立即退出循环
- const，定义常量或者不变裸指针constan raw pointer
- contine，继续进入下一次循环迭代
- crate，在模块路径中代指crate root
- dyn，动态分发trait对象
- else，作为if 和if else 控制流结构的fallback
- enum
- extern，链接一个外部函数或变量
- false，
- fn，定义一个函数或者函数指针类型
- if
- impl 实现自有或者trait功能
- in， for 循环语法的一部分
- let
- loop，无条件循环
- match，模式匹配
- mod，定义一个模块
- move，使闭包获取其所捕获项的所有权
- mut，表示引用、裸指针或者模式绑定的可变性
- pub，表示结构体、impl块或模块的公有可见性
- ref，通过引用绑定
- return 从函数中返回
- Self，定义或实现trait的类型的类型别名
- self，表示方法本身或当前模块
- static，表示全局变量或在整个程序执行期间保持其生命周期
- struct
- super，表示当前模块的父模块
- trait，定义一个trait
- true
- type，定义一个类型别名或关联类型
- union，定义一个union并且是union声明中唯一用到的关键字
- unsafe
- use，引入外部空间的符号
- where，表示一个约束类型的从句
- while，基于一个表达式的结果判断是否进行循环

## 保留做将来使用的关键字

- abstract
- become
- box
- do
- final
- macro
- overrid
- priv
- try
- typeof
- unsized
- virtual
- yield

## 原始标识符

- `r#`

```rs
// error
fn match(needle: &str, haystack: &str) {
  // code
}

fn r#match(needle: &str, haystack: &str) {
  // code
}
```
