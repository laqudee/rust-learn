# 标准库类型

- 部分自定义类型：
  - String
  - vector
  - 选项类型optional types, option<i32>
  - 错误处理类型error handling types, Result<u32, i32>
  - 堆分配的指针heap allocated pointers, Box<i32>

# 箱子、栈、堆

- Box<T>

- 被装箱的值可以使用*运算符解引用，这会移除一层装箱

# 动态数组vector

- vector是大小可变的数组。和slice切片类似，其大小在编译时未知，但他们可以随时扩大或缩小

- 使用3个单词来表示：一个指向数据的指针、vector的长度、它的容量：指明要为这个vector保留多少内存

# 字符串

- String：被存储为由字节组成的vector（Vec<u8>），在堆上，不是零结尾的
- &str：指向有效UTF-8序列的切片（&[u8]），可用来查看String的内容

# 选项Option

- None，表明失败或缺少值
- Some(value)，元组结构体，封装了一个T类型value

# 结果Result
- Result<T, E>
  - Ok(value)
  - Err(why)


# ? 运算符
- 把result 用match连接起来会显得很难看，可以使用？运算符可以把这个逻辑变得干净漂亮。

# panic!

- panic!宏可用于产生一个panic，并开始回退（unwind）它的栈。在回退栈 的同时，运行时将会释放该线程所拥有的所有资源，这是通过调用线程中所有对象的 析构函数完成的

# 散列表HashMap
- vector通过整型下标来存储值，而HashMap散列表通过键来存储值
- HashMap的键可以是布尔型、整型、字符串、或任意实现了Eq 和 Hash trait

- 和 vector 类似，HashMap 也是可增长的，但 HashMap 在占据了多余空间时还可以缩小 自己。可以使用 HashMap::with_capacity(unit) 创建具有一定初始容量的 HashMap，也 可以使用 HashMap::new() 来获得一个带有默认初始容量的 HashMap（这是推荐方式）
