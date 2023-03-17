#  Containers and collections

- error-handling types
  - option, Option<T>
  - result, Result<T, E>
 
 - iter module defines Rust's iterator trait, Iterator
   - for loop

- Vec<T>
- [T;N]
- [T], A dynamically slice into any other kind of contigous storage,whether heap-alloccated or not.

- &[T], shared slice
- &mut [T], mutable slice
- Box<[T]>, owned slice

- str, a UTF-8 string slice, 
  - &str

- String, buliding and mutating string.

- format! macro

- box or Rc type
- Cell or RefCell
- Arc Mutex
- HashMap<K, V>

# Platform abstractions and I/O

- Common types of I/O: 
  - files
  - TCP
  - UDP
  - defined in the io, fs, and net modules

- thread module contains rust's threading abstractions.
- sync contains further primitive shared memory types, including atomic and mpsc, which contains the channel types for message passing.

# Primitive Types

- never, the ! type, also calleds "never".
- array, a fixed array, [T;N]
- bool
- char
- f32, f64
- fn
- i8, i16, i32, i64, isize
- pointer, Raw unsafe pointers, *const T, *mut T.
- reference, &T, &mut T
- slice
- str
- tuple, (T, U, ...)
- u8, u16, u32, u64, u128, unit, usize

# Modules

- alloc
- any
- arch
- array
- ascii
- backtrace
- borrow, a  module for working with borrwed data
- boxed, the Box<T>type for heap allocation(堆上的内存分配)
- cell, shareable mutable containers
- char
- clone, the Clone trait for types that cannot be implicitly copied 
- cmp, 比较comparing与排序ordering
- collections, collection types
- convert, Traits for conversions between types
- default, the default trait for types with default value
- env, inspection and manipulation of the process's enbironment.
- error, interfaces for working with Errors
- ffi,
- fmt, for formatting and printing Strings
- fs, filesystem 操作
- future, asynchronous basic functionality
- hash, generic hashing support
- hint,
- io, Traits, helpers, and type definitions for core I/O functionality
- iter, 可组合的外部迭代
- marker
- mem, basic functions for dealing with memory
- net, networking primitives for TCP/UDP communication
- num, additional functionality for numerics
- ops, overloadable operators
- option, optional values
- os, OS-specific functionality
- panic,
- path, cross-platform 操作
- pin,
- prelude, the rust prelude
- primitive,

- process, a module for working with process
- ptr
- rc, single-threaded reference-counting pointers. 'RC' stands for Reference Counted
- result,
- slice
- str
- string
- sync
- task
- thread, native threads
- time, temporal quantification
- vec, a contiguous growable array type width heap-allocated contents, written Vec<T>

# Mcros
- assert
- assert_eq
- assert_ne
- cfg
- column
- compile_error
- concat
- dbg
- debug_assert
- debug_assert
- debug_assert_ne
- env
- eprint
- eprintln
- file
- format
- format_args
- include
- include_bytes
- include_str
- line
- mathes
- module_path
- option_env
- panic
- print
- println
- stringify
- thread_local
- todo
- unimplemented
- unreachable
- vec, creates a Vec containing the aarguments.
- write, write formatted data info a buffer,
- writeln, write formatted data into a buffer, with a newline appended.