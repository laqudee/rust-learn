# bool

- true, 1
- false, 0

## Basic Usage

- 位运算， BitAnd, BitOr, Not, etc.
- if 逻辑运算

```rs
let bool_val = true & false | false;

assert!(!bool_val);
```

## implementations

- impl bool

- pub fn then_some<T>(self, t: T) -> Option<T>

- pub fn then<T, F>(self, f: F) -> Option<T> where F: FnOnce() -> T

