# str

- the str type, also called a 'string slice', is the most primitive string type.
- It is usually seen in its borrowed from, &str.
- &'static str

- the internals of &str, unsafe should not be used to get a string slice under normal circumstances. use as_str instead.

## Implementations

- impl str

- pub const fn len(&self) -> usize

- pub const fn is_empty(&self) -> bool

- pub fn is_char_boundary(&self, index: usize) -> bool

- pub fn floor_char_boundary(&self, index: usize) -> usize

- pub const fn as_byte(&self) -> &[u8]

- pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8]

- pub const fn as_ptr(&self) -> *const u8

- pub fn as_mut_ptr(&mut self) -> *mut u8

- pub fn get<I>(&self, i: I) -> Option<&<I as SliceIndex<str>>::Output> where I: SliceIndex<str>,

- pub fn get_mut<I>(&mut self, i: I) -> Option<&mut I>

- pub unsafe fn get_unchecked<I>(&self, i: I) -> &<I as SliceIndex<str>>::Output where I: SliceIndex<str>,

- pub unsafe fn get_unchecked<I>(&mut self, i: I) -> &mut <I as SliceIndex<str>>::Output where I: SliceIndex<str>,

- pub fn split_at(&self, mid: usize) -> (&str, &str)
  - divide one string slice into two at an index

- pub fn split_at_mut(&mut self, mid: usize) -> (&mut str, &mut str)

- pub fn chars(&self) -> Chars<'_'>
  - returns an iterator over the chars of a string slice.
  - chars might not match your intuition about characters

- pub fn char_indices(&self) -> CharIndices<'_'>

- pub fn bytes(&self) -> Bytes<'_>
  - an iterator over the bytes of a string slice.

- pub fn split_whitespace(&self) -> SplitWhitespace<'_>
  - split a string slice by whitespace