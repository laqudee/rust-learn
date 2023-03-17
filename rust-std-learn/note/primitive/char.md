# char

## implementation

- impl char

- pub const MAX: char = '\u{10ffff}'

- pub const REPLACEMENT_CHATACTER:char = '�'

- pub const UNICODE_VERSION: (u8, u8, u8) = crate::unicode::UNICODE_VERSION

- pub fn decode_utf16<I>(iter: I) -> DecodeUtf16<<I as IntoIterator>::IntoIter> where I: IntoIterator<Item = u16>,