# 泛型

- 泛型是关于泛化类型和函数功能以扩大适用范围的话题

- 泛型最简单和常用的用法就是用于类型参数

# 函数
- 同样的规则也适用于函数：在使用类型T前给出`<T>`，那么就变成了泛型

- 调用泛型函数有时需要显式指明类型参量

# 实现
- 和函数类似，impl块要想实现泛型，也需要仔细

# trait

- trait也可以是泛型，我们定义一个trait，它把Drop trait作为泛型方法实现了，可以drop调用者本身和一个输入参数

# 约束

- 在使用泛型时，类型参数常常比心使用trait作为约束bound来明确规定类型应实现哪些功能。

- 下一个例子：用到了Display trait 来打印，所以它用Display来约束T，也就是T必须实现Display

- 约束把泛型类型限制为符合约束的类型

```rs
fn printer<T: Display>(t: T) {
  println!("{}", t);
}

struct s<T: Display>(T);

// error, Vec<T>没有实现Display trait
let s = S(vec![1]);
```

- 约束的另一个作用是泛型的实例可以访问作为约束的trait的方法

- 某些情况下，也可以使用where 分句来形成约束，者拥有更好的表现力

# 测试实力：空约束

- 约束的工作机制会产生这样的效果：即使一个 trait 不包含任何功能，你仍然可以用它 作为约束。标准库中的 Eq 和 Ord 就是这样的 trait。

# 多重约束

- 多重约束multiple bounds可以用 + 连接，和平常一样，类型之间使用,隔开

# where分句

- 约束可以使用where分句来表达，它放在{之前
- where从句可以用于任意类型的限定而不局限于类型参数本身

```rs
impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}

// 使用 `where` 从句来表达约束
impl <A, D> MyTrait<A, D> for YourType where
    A: TraitB + TraitC,
    D: TraitE + TraitF {}

```

# new type 惯用法

- newtype惯用法（为不同种类的数据分别定义新的类型）能保证在编译时，提供给程序的都是正确的类型

# 关联项

- 与多种类型的项有关的一组规则。是trait泛型的扩展，允许在trait内部定义新的项

- 当trait对于实现了它的容器类型是泛型，关联项就是提供了简单的使用方法

# 存在问题

- trait 如果对实现了它的容器类型是泛型的，则须遵守类型规范要求——trait 的 使用者必须指出 trait 的全部泛型类型

- 关联类型
- 通过把容器内部的类型放到trait中作为输出类型，使用关联类型增加了代码的可读性

# 虚类型参数
- 虚类型（phantom type）参数是一种在运行时不出现，而在（且仅在）编译时进行静态检查 的类型参数。

可以用额外的泛型类型参数指定数据类型，该类型可以充当标记，也可以供编译时类型检查 使用。这些额外的参数没有存储值，也没有运行时行为。
