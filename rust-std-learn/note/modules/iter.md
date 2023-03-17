# Module std::iter

- composable external iteration

## Organization
- this module is largely organized by type.
  - Trait are the core portion
  - Functions provide some helpful ways to create some basic iterators
  - Structs

```rs
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

- iter()
- iter_mut()
- into_iter()

## Implementing Iterator

## for loops and IntoIterator
