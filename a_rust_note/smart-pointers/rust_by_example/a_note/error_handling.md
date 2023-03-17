# 处理多种错误类型

- 有时也需要Option和Result进行交互，或是Result<T, Error1>需要和Result<T, Error2>进行交互

# 从Option中取出Result
- 处理混合错误类型的最基本的手段就是让他们互相包含

- 有时我们不在想处理错误（比如使用?），但如果Option是None则继续处理错误，一些组合算子可以让我们轻松地交换Result和Option

# 定义一个错误类型
- 一个好的错误类型应当：
  - 用同一个类型代表多种错误
  - 向用户提供清楚的错误信息
  - 能够容易地与其他类型比较
  - 能够容纳错误的具体信息
  - 能够与其他错误很好的整合

# 把错误装箱

- 如果又想写简单的代码，又想保存原始错误信息，一个方法就是把他们装箱Box
- 坏处就是被包装的错误类型只能在运行时了解，而不能被静态地判别

- 对任何实现了 Error trait 的类型，标准库的 Box 通过 From 为它们提供了 到 Box<Error> 的转换

# ?的其他用法

- ?之前被解释为要么unwrap，要么retrun Err(err)，这只是在大多数情况下是正确的。

- ?实际上是指unwrap或retrun Err(From::from(err))

# 包裹错误

- 把错误装箱这种做法也可以改成把它包裹到你自己的错误类型中

# 遍历Result

- Iter::map()操作可能失败

- filter_map()忽略失败的项
  - 调用一个函数，过滤掉为None的所有结果

- collect()使整个操作失败
  - Result 实现了 FromIter，因此结果的向量（Vec<Result<T, E>>）可以被转换成 结果包裹着向量（Result<Vec<T>, E>）。一旦找到一个 Result::Err ，遍历就被终止
  - 同样的技巧可以对Option使用

- Partition()收集所有合法的值和错误

- 
