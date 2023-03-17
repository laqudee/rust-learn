- 枚举允许通过列举可能的成员来定义一个类型

- Option

- if let

- 结构体给予你将字段和数据聚合在一起的方法

- 枚举给予你将一个值成为一个集合之一的方法。

# enum

```rs
  enum IpAddrKind {
    V4,
    V6,
  }

  let four = IpAddrKind::V4;
  let six = IpAddrKind::V6;

  fn route(ip_kind: IpAddrKind) {}

  route(IpAddrKind::V4);

  struct IpAddr {
    kind: IpAddrKind,
    address: String,
  }

  let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
  };

  let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
  };

  enum IpAddr {
    V4(String),
    V6(String),
  }

  let home = IpAddr::V4(String::from("127.0.0.1"));
  let loopback = IpAddr::V6(String::from("::1"));

```

```rs
  enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
  }

  let home = IpAddr::V4(127, 0, 0, 1);
  let loopback = IpAddr::V6(String::from("::1"));

  struct Ipv4Addr {
    // --snip--
  }

  struct Ipv6Addr {
    // --snip
  }

  enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
  }
```

- 可以将任意类型的数据放入枚举成员中：例如字符串、数字类型或者结构体。

```rs
enum Message {
  Quit, // 没有关联任何数据
  Move {x: i32, y: i32}, // 类似结构体包含命名字段
  Write(String),
  ChangeColor(i32, i32, i32),
}

struct QuitMessage; // 类单元结构体
struct MoveMessage {
  x: i32,
  y: i32,
}

struct WriteMessage(String); // 元组结构体
struct ChangeColorMessage(i32, i32, i32); // 元组结构体
```

- 结构体和枚举另一个相似点：impl 定义方法

```rs
impl Message {
  fn call(&self) {
    // 在这里定义方法体
  }
}

let m = Message::Write(String::from("hello"));
m.call()
```

# option

- Option枚举和其相对于空值的优势

- 即一个值要么有值要么没有值

- 变量总有两种状态：空值和非空值

- 空值是一个因为某种原因目前无效或缺失的值

- rust没有空值，但它拥有一个可以编码存在或不存在概念的枚举。

```rs
enum Option<T> {
  None,
  Some(T),
}
```

- 泛型参数

```rs
let some_number = Some(5);
let some_char = Some('e');

let absent_number: Option<i32> = None;
```

- `Option<T>` 和 `T`（这里 `T` 可以是任何类型）是不同的类型，编译器不允许像一个肯定有效的值那样使用 `Option<T>`

- 在对`Option<T>`进行T的运算之前必须将其转换为T


