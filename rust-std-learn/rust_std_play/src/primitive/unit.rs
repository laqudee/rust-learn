use std::io::*;

pub fn play() {
    // impl FromIterator<()> for ()
    let data = vec![1,2,3,4,5];
    let res: Result<()> = data.iter().map(|x| writeln!(stdout(), "{x}")).collect();

    assert!(res.is_ok());
}