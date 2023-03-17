# hash map 储存键值对

- `HashMap<k, v>`类型储存了一个键类型k对应一个值类型v的映射。
  - 通过一个哈希函数来实现，决定如何将键和值放入内存中

```rs
// 新建
// new 创建一个空的HashMap并使用insert增加元素
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

- 哈希map将它们的数据存在堆上。

- 类似于vector，哈希map是同质的：所有的键必须是相同类型，值也必须都是相同类型

- 另一个构建哈希map的方法是在一个元组的vector上使用迭代器iterator和collect方法

- zip

```rs
use std::collections::HashMap;

let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect()

// HashMap<_, _>注解是必要的
```

- 哈希map和所有权
  - 对于像i32这样实现了Copy trait的类型，其值直接拷贝进哈希map
  - 对于像String这样拥有所有权的值，其值将被移动而哈希map会成为这些值的所有者

```rs
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new()
map.insert(field_name, field_value);
// 这里field_name和field_value不再有效
```

- 生命周期与引用有效性

- 访问哈希map中的值

```rs
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::form("Blue");

let score = scores.get(&team_name); // Some(10) , or None

for (key, value) in &scores {
  println!("{}: {}", key, value)
}
```

- 更新哈希Map

```rs
use std::collections::HashMap;

let mut scores = HashMap::new();

// 覆盖一个值
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores); // {"Blue": 25}

// 只在键没有对应值时插入 entry() or_insert()
scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50)

println!("{:?}", scores); // {"Blue": 25, "Yellow": 50}

// 根据旧值更新一个值
let text = "hello world wonderful world";
let mut map = HashMap::new();
for word in text.split_whitespace() {
  let count = map.entry(word).or_insert(0);
  *count += 1;
}
println!("{:?}", map); // {"world": 2, "hello": 0, "wonderful": 1} 

// or_insert()返回这个键的值的一个可变引用(&mut V)
// *  解引用
```

- SipHash
