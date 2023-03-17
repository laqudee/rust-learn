- use，我们可以使用use关键字将路径一次性引入作用域，然后调用该路径中的项

- 在作用域中增加use和路径类似于在文件系统中创建软连接（符号连接，symbolic link）。
  - 通过在crate根增加`use crate::front_of_house::hosting`，现在hosting在作用域中就是有效的名称了。

- 还可以使用 use 和相对路径来将一个项引入作用域

- 创建惯用的use路径
  - 使用use将add_to_waitlist函数引入作用域，这并不符合习惯

- 使用use引入结构体、枚举和其他项时，习惯是指定它们的完整路径

```rs
use std::collections::HashMap;

fn main() {
  let mut map = HashMap::new();
  map.insert(1, 2);
}
```

```rs
use std::fmt;
use std::io;

fn function1() -> fmt::Result {}

fn function2() -> io::Result<()> {}
```

- 使用as关键字提供新的名称

```rs
use std::fmt::Result;
use std::io::Result as IoResult;

fn function
```

- 使用pub use 重导出名称

- 重导出：不仅将一个名称导入了当前作用域还允许别人把它导入自己的作用域

- 当你的代码的内部结构与调用你代码的程序员所想象的结构不同时，重导出很有用。

- 使用外部包

```rs
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--

use std::{cmp::Ordering, io};

use std::io;
use std::io::Write;

use std::{self, Write};
```

- 通过glob运算符将所有的公有定义引入作用域
  - 经常用于tests中

```rs
use std::collections::*
```
