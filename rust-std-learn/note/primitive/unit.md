# unit

- the () type also called "unit"

- the () type has exactly one value (), and is used when there is no other meaningful value that could be returned.
- () is most commonly seen implicitly: functions without a -> ... impliaitly have return type (), that is, these are equivalent

```rs
fn long() -> () {}

fn short() {}

fn returns_i64() -> i64 {
    1i64
}

fn returns_unit() {
    1i64;
}

let is_i64 = {
    returns_i64()
};

let is_unit = {
    return_unit();
};
```