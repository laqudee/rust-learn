# pointer

- Raw unsafe pointer, *const T, snd *mut T

- raw pointers(原始指针) can be unaligned or null

- using the * operator

- *const T
- *mut T

## Common ways to create raw pointer

- coerce a reference(&T) or mutable reference(&mut T)

- Consume a box(Box<T>)
  - into_raw(): consumes a  box and returns the raw pointer(消费一个box，返回一个原始指针). It doesn't destory T or deallocate any memory(不销毁值且不释放任何内存)

- drop(): it indicates that we are done with the given value and it should be destoryed.

- Create it using ptr::addr_of!

- Get it from C
  - usually you wouldn't literally use malloc and free from Rust, but C APIs hand out a lot of pointers generally, so are a common source of raw pointers in Rust

## Implementations

- impl <T> *const [T]

- pub fn len(self) -> usize

- pub fn as_ptr(self) -> *const T

- pub unsafe fn get_unchecked<I>(self, index: I) -> *const <I as SliceIndex<[T]>>::Output where I: SliceIndex<[T]>,

- impl<T> *const T where T: ?sized,

- pub fn is_null(self) -> bool

- pub const fn cast<U>(self) -> *const U

- pub unsafe fn as_ref<'a>(self) -> Option<&'a T>

- pub unsafe fn offset(self, count: isize) -> *const T

- pub const fn wrapping_offset(self, count: isize) -> *const T

- pub const unsafe fn offset_from(self, origin: *const T) -> isize

- pub const unsafe fn add(self, count: usize) -> *const T

- pub const fn wrapping_add(self, count: usize) -> *const T

- impl<T> *mut T

- pub fn is_null(self) -> bool