# if let 简洁控制流

- 解决冗长的if与let

```rs
let config_max = Some(3u8);
match config_max {
  Some(max) => println!("The maximum is configured to be {}", max),
  _ => (),
}

// 使用if let 与上述代码行为一致
if let Some(max) = config_max {
  println!("The maxiumum is configured to be {}", max);
}
```

- `if let`语法获取通过等号分隔的一个模式和一个表达式。工作方式与match相同，这里的表达式对应match模式的第一个分支

- match与if let之间要在安全检查与简洁之间取舍

- if let 就是match的语法糖

```rs
let mut count = 0;
match coin {
  Coin::Quarter(state) => println!("dddd"),
  _ => count += 1,
}

// 使用 if let and else
let mut count = 0;
if let Coin::Quarter(state) = coin {
  println!("dddd");
} else {
  count += 1;
}
```
