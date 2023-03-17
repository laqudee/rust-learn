- 标量
- 复合

- rust是静态类型语言，在编译时必须知道所有变量的类型。

- 标量，代表一个单独的值
  - 整型
  - 浮点型
  - 布尔型
  - 字符类型

- 数值运算
  - 加法、减法、乘法、除法、取余

- 布尔型： true false

- 字符型
  - char类型是语言中最原生的字母类型，char大小为四个字节，比ascll表示更多的内容，unicode

```rs
  fn main() {
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻'
  }
```

- 复合类型
  - 可以将多个值组成一个类型

- rust有两个原生复合类型：元组tuple和数组array

- 元组：将一个或多个其他类型的值组合进一个复合类型的主要方式。长度固定，不可增大或缩小。

- 为了从元组中获取单个值，可以使用模式匹配pattern matching；来结构元组值

```rs
  fn main () {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;
  }
```

- 不含有任何值的元组有一个特殊的名称，叫作单元元组 (), 空值、空的返回值

- 数组类型
  - 数组中的每个元素的类型必须相同。且数组长度是固定的

```rs
  fn main () {
    let a = [1, 2, 3, 4, 5];

    let b: [i32; 5] = [1, 2, 3, 4, 5];

    let c = [3; 4]; // [3,3,3,3]

    let first = a[0]
  }
```

- vector类型，允许增长或缩小类似数组的集合的长度。当不确定使用数组还是vector时，应该使用vector

- 无效的数组访问
