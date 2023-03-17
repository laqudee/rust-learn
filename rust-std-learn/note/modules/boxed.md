# Modules std::boxed

- The Box<T> type for heap allocation

- Box<T>, casually refered to as a 'box',provides the simplest form of heap allocation in rust.
- Boxes provide ownership for this allocation, and drop their contents when they go out of scope.

## Memory layout

- for non-zero-sized values, a Box will use the Global allocator for its allocation.

