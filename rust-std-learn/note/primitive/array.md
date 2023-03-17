# Array

- [x, y, z]
- [T; N]; which produces an array with N copied of x, the type of x must be Copy

- [expr; 0] is allowed, and productuces an empty array.

- Arrays of any size implement the following traits if the element type allows it:
  - Copy
  - Clone
  - Debug
  - IntoIterator(implemented for [T; N], &[T; N], &mut [T; N])
  - PartialEq, PartialOrd,Eq, Ord
  - Hash
  - AsRef, AsMut
  - Borrow, BorrowMut

- slices([T])
  - a dynamically-size view into a contiguous sequence(一个动态大小的相邻顺序)[T]
  - Contiguous here meanss that elements are laid out so that every element is the same distance form its neighbors

- can move elements out of an array with a slice pattern. if want one element, see mem::repalce.

## Editions

- Rust 2015 and 2018:
  - arrays did not implement IntoIterator by value, so that the method call array.into_iter() auto-referenced into a slice iterator.

  - iter()值引用（借用）
  - IntoIterator::into_iter(array), 值的所有权
  - for it in array {}, 值的所有权
  - for item in IntoIterator::into_iter(array).enumerate()，值的所有权

- Rust 2021:
  - array.into_iter() uses IntoIterator normally to iterate by value, and iter() should be used to iterate by reference like previous editions

  - into_iter() 需要值的所有权
  - iter() 需要的是值的引用（借用）

## Implementations

- pub fn map<F, U>(self, f: F) -> [U; N];

- In many cases, can instead use Iterator::map by calling .iter()  or .into_iter() on your array.
- [T; N]::map is only necessary if you really need a new array of the same size as the result.

```rs
pub fn try_map(F, R)(self, f: F) -> <<R as Try>::Residual as Residual<[<R as Try>::Output; N]>>::TryType 
where
    F: FnMut(T) -> R,
    R: Try,
    <R as Try>::Residual: Residual<[<R as Try>::Output; N]>,
```

- pub fn zip<U>(self, rhs: [U; N]) -> [(T, U); N];
  - 将两个数组转换成元组

- pub const fn as_slice($self) -> &[T]
  - Return a slice containing the entrie array. equivalent to &s[...]

- pub fn as_mut_slice(&mut self) -> &mut [T]
  - Returns a mutable slice containing the entire array. Equivalent to &mut s[..]

- pub fn each_ref(&self) -> [&T; N]

```rs
let floats = [3.1, 2.7, -1.0];
let float_refs = floats.each_ref();
assert_eq!(float_refs, [&3.1, &2.7, &-1.0]);

let strings = ["Ferris".to_string(), "♥".to_string(), "Rust".to_string()];
let is_ascii = strings.each_ref().map(|s| s.is_ascii());
assert_eq!(is_ascii, [true, false, true]);

assert_e1!(strings.len(), 3);
```

- pub fn each_mut(&mut self) -> [&mut T; N]

```rs
let mut floats = [3.1, 2.7, -1.0];
let float_refs: [&mut f64; 3] = floats.each_mut();
*float_refs[0] = 0.0;
assert_eq!(float_refs, [&mut 0.0, &mut 2.7, &mut -1.0]);
assert_eq!(floats, [0.0, 2.7, -1.0]);
```

- pub fn split_array_ref<const M: usize>(&self) -> (&[T; M], &[T])
  - 从左边开始

- pub fn split_array_mut<const M: usize>(&mut self) -> (&mut [T; M], &mut [T])
  - 从左边开始

- pub fn rsplit_array_ref<const M: usize>(&self) -> (&[T], &[T; M])
  - 从右边开始

- pub fn rsplit_array_mut<const M: usizze>(&mut self) -> (&mut [T; M] &mut [T])
  - 从右边开始