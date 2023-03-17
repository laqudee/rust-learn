fn basic_play() {
    let _hello_world = "Hello Word";
    let _hello_world: &'static str = "Hello Rust";

    use std::slice;
    use std::str;

    let story = "Once upon a time...";

    let ptr = story.as_ptr();
    let len = story.len();

    assert_eq!(19, len);

    let s = unsafe {
        let slice = slice::from_raw_parts(ptr, len);
        // let slice = slice::from_raw_parts(ptr, len-7); // OK("Once upon a ")

        str::from_utf8(slice)
    };

    println!("s: {:?}", s);
    assert_eq!(s, Ok(story));
}

fn impl_str_play() {
    use std::str;

    let len = "foo".len();
    assert_eq!(3, len);

    assert_eq!("ƒoo".len(), 4); // fancy f!
    assert_eq!("ƒoo".chars().count(), 3);

    let s = "";
    println!("s is empty: {}", s.is_empty());

    {
        let s = "Löwe 老虎 Léopard";
        println!("s.is_char_boundary(0): {:?}", s.is_char_boundary(0));

        assert!(s.is_char_boundary(6));
        assert!(s.is_char_boundary(s.len()));

        assert!(!s.is_char_boundary(2));
        assert!(!s.is_char_boundary(8));
    }

    let bytes = "bors".as_bytes();
    println!("bytes: {:?}", bytes); // bytes: [98, 111, 114, 115]
    let str_from_bytes = str::from_utf8(bytes);
    println!("str_from_bytes: {:?}", str_from_bytes);
    assert_eq!(b"bors", bytes);
    assert_eq!("bors", str_from_bytes.unwrap());

    let mut s = String::from("🗻∈🌏");
    unsafe {
        let bytes = s.as_bytes_mut();

        bytes[0] = 0xF0;
        bytes[1] = 0x9F;
        bytes[2] = 0x8D;
        bytes[3] = 0x94;
    }
    println!("mulability s is {:?}", s);

    let s = "Hello";
    let ptr = s.as_ptr();
    println!("ptr is {:?}", ptr);
    let s2: &mut str = &mut "Hello".to_string();
    let ptr2 = s2.as_mut_ptr();
    println!("ptr is {:?}", ptr2);

    let v = String::from("🗻∈🌏");
    println!("v get index = 0..4 is {:?}", v.get(0..4));

    // indices not on UTF-8 sequence boundaries
    println!("v.get(1..) is none: {}", v.get(1..).is_none());
    assert!(v.get(..8).is_none());

    // out of bounds
    assert!(v.get(..42).is_none());

    {
        let mut v = String::from("hello");
        assert!(v.get_mut(0..5).is_some()); // true
        assert!(v.get_mut(..42).is_none());
        let he = v.get_mut(0..2).map(|v| &*v);
        println!("he: {:?}", he);

        {
            let s = v.get_mut(0..2);
            let s = s.map(|s| {
                s.make_ascii_uppercase();
                &*s
            });

            println!("s: {:?}", s);
        }
    }

    {
        let v = "🗻∈🌏";
        println!("length of v: {}", v.len()); // bytes 11
        unsafe {
            println!("v.get_unchecked(0..4): {:?}", v.get_unchecked(0..4)); // bytes 4
            println!("v.get_unchecked(4..7): {:?}", v.get_unchecked(4..7)); // bytes 3
            println!("v.get_unchecked(7..11): {:?}", v.get_unchecked(7..11)); // bytes 4
        }
    }

    {
        let mut v = String::from("🗻∈🌏");
        unsafe {
            assert_eq!("🗻", v.get_unchecked_mut(0..4));
            assert_eq!("∈", v.get_unchecked_mut(4..7));
            assert_eq!("🌏", v.get_unchecked_mut(7..11));

            println!(
                "v.get_unchecked_mut(7..11): {:?}",
                v.get_unchecked_mut(7..11)
            );
        }
    }

    {
        let s = "Per Martin-Löf";

        let (first, last) = s.split_at(3);
        println!("first: {:?}; last: {:?}", first, last);

        let mut s = "Per Martin-Löf".to_string();
        {
            let (first, last) = s.split_at_mut(3);
            first.make_ascii_uppercase();
            println!("first: {:?}; last: {:?}", first, last);
        }
    }

    let word = "goodbye";
    let count = word.chars().count();
    assert_eq!(7, count);

    let mut chars = word.chars();
    assert_eq!(Some('g'), chars.next());
    assert_eq!(Some('o'), chars.next());
    assert_eq!(Some('o'), chars.next());
    assert_eq!(Some('d'), chars.next());
    assert_eq!(Some('b'), chars.next());
    assert_eq!(Some('y'), chars.next());
    assert_eq!(Some('e'), chars.next());

    assert_eq!(None, chars.next());

    {
        let y = "y̆";
        let mut chars = y.chars();
        println!("chars: {:?}", chars); // chars: Chars(['y', '\u{306}'])
        println!("chars.next(): {:?}", chars.next());
        println!("chars.next(): {:?}", chars.next());
        println!("chars.next(): {:?}", chars.next());
    }

    {
        let word = "goodbye";

        let count = word.char_indices().count();
        assert_eq!(7, count);

        let mut char_indices = word.char_indices();

        assert_eq!(Some((0, 'g')), char_indices.next());
        assert_eq!(Some((1, 'o')), char_indices.next());
        assert_eq!(Some((2, 'o')), char_indices.next());
        assert_eq!(Some((3, 'd')), char_indices.next());
        assert_eq!(Some((4, 'b')), char_indices.next());
        assert_eq!(Some((5, 'y')), char_indices.next());
        assert_eq!(Some((6, 'e')), char_indices.next());

        assert_eq!(None, char_indices.next());
    }

    let mut bytes = "bors".bytes();
    println!("bytes.next(): {:?}", bytes.next()); // bytes.next(): Some(98)
                                                  // assert_eq!(Some(b'b'), bytes.next());

    let mut iter = "A few words".split_whitespace();
    assert_eq!(Some("A"), iter.next());
    assert_eq!(Some("few"), iter.next());
    assert_eq!(Some("words"), iter.next());

    assert_eq!(None, iter.next());
}

pub fn play() {
    basic_play();
    impl_str_play();
}
