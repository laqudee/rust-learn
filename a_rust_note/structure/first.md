# 结构体

- 定义和实例化结构体
- 定义关联函数
- 方法
- 在程序中基于结构体和枚举enum创建新类型

> structure，是一个自定义数据类型，允许包装和命名多个相关的值，从而形成一个有意义的组合

```rs
  struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
  }

  fn main() {
    let mut user1 = User {
      email: String::from("zyl@geo.com"),
      username: String::from("zyl"),
      active: true,
      sigin_in_count: 1,
    };

    user1.email = String::from("another@geo.com");
  }
```

- 整个实例必须是可变的；rust并不允许只将某个字段标记为可变。

- 使用结构体更新语法从其他实例创建实例
  - struct update syntax

```rs
  fn build_user(email: String, username: String) -> User {
    User {
      email: email,
      username: username,
      active: 1,
      sigin_in_count: 1,
    }
  }

  // 使用字段初始化简写语法
  fn build_user(email: String, username: String) -> User {
    User {
      email,
      username,
      active: 1,
      sigin_in_count: 1,
    }
  }

  fn main() {
    // --snip--
    let use1 = build_user("zyl@geo.com", "zyl");

    let user2 = User {
      active: user1.active,
      username: user1.username,
      email: String::from("another@geo.com"),
      sign_in_count: user1.sigin_in_count,
    }

    // or ..语法
    let user2 = User {
      email: String::from("zzz@geo.com"),
      ..user1
    }
  }
```

- 结构更新语法就像带有 = 的赋值，因为它移动了数据。
  - 创建了user2之后不能再使用user1，因为user1的username字段中的String被移动到user2中。
  - 如果我们给 user2 的 email 和 username 都赋予新的 String 值，从而只使用 user1 的 active 和 sign_in_count 值，那么 user1 在创建 user2 后仍然有效。active 和 sign_in_count 的类型是实现 Copy trait 的类型

- 使用没有命名字段的元组结构来创建不同的类型
  - 元组结构体tuple structs，没有具体字段名，只有字段类型

```rs
  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);

  fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
  }

  // black 和 origin 值的类型不同，因为它们是不同的元组结构体的实例
  // 也可以使用.后跟索引来访问单独的值
```

- 没有任何字段的类单元结构体
  - 类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用

```rs
  struct AlwaysEqual;

  fn main() {
    let subject = AlwaysEqual;
  }
```

- 结构体数据的所有权
  - 生命周期确保结构体引用的数据有效性跟结构体保持一致


