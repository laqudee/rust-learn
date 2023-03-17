pub fn play() {
    let c: char = 'a';
    match c {
        '\0'..='\u{D7FF}' => println!("false"),
        '\u{E000}'..='\u{10FFFF}' => println!("true"),
    }

    char_representation();

    impl_char();
}

fn char_representation() {
    let v = vec!['h', 'e', 'l', 'l', 'o'];
    println!(
        "five elements tims four bytes for each element: {}",
        v.len() * std::mem::size_of::<char>()
    );

    let s = String::from("hello");
    println!(
        "five elements times one byte per element: {}",
        s.len() * std::mem::size_of::<u8>()
    );

    let s = String::from("love: ❤️");
    let v: Vec<char> = s.chars().collect();
    println!("the length of s is {}", std::mem::size_of_val(&s[..]));
    println!("the length of v is {}", std::mem::size_of_val(&v[..]));
}

fn impl_char() {
    fn something_which_returns_char() -> char {
        'a'
    }

    let c = something_which_returns_char();
    assert!(c <= char::MAX);

    let value_at_max = char::MAX as u32;
    assert_eq!(char::from_u32(value_at_max), Some('\u{10FFFF}'));
    assert_eq!(char::from_u32(value_at_max + 1), None);
}
