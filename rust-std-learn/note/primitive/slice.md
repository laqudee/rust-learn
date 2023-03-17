# slice

- A dynamically-sized view into a contiguous sequence(一个动态大小的连续序列), [T].

- Contiguous here means that elements are laid out so that every element is the same distance from its neighbors.

- Slices are a view into a block of memory represented as a pointer and a length;

## Implementations

- impl <T> Box<T, Global>

- impl <T> [T]

- pub const fn len(&self) -> usize

- pub const fn is_empty(&self) -> bool

- pub const fn first(&self) -> Option<&T>

- pub fn first_mut(&mut self) -> Option<&mut T>

- pub const fn split_first(&self) -> Option<(&T, &[T])>

- pub fn split_first_mut(&mut self) -> Option<(&mut T, &mut [T])>

- pub const fn split_last(&self) -> Option<(&T, &[T])>

- pub fn split_last_mut(&mut self) -> Option<(&mut T, &mut [T])>

- pub const fn last(&self) -> Option<&T>

- pub fn last(&mut self) -> Option<&mut T>

- ```rs
    pub fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[T]>>::Output>
    where
        I: SliceIndex<[T]>,
  ```

- ```rs
    pub fn get_mut<I>(&mut self, index: I) -> Option<&mut <I as SliceIndex<[T]>>::Output>
    where
        I: SliceIndex<[T]>,
  ```

- pub unsafe fn get_unchecked<I>(&self, index: I) -> &T

- pub unsafe fn get_unchecked_mut<I>(&mut self, index: I) -> &mut T

- pub const as_ptr(&self) -> *const T

- pub const fn as_mut_ptr(&mut self) -> *mut T