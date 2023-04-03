use std::io::{BufReader, Cursor, Read};

fn main() {
    let mut buf = [0u8; 10];

    // A buffered reader decorated a vector reader which wraps input data
    let mut input = BufReader::new(Cursor::new("Input Data"));

    input.read(&mut buf).ok();

    print!("Read from a buffered reader: ");

    for byte in buf {
        print!("{}", char::from(byte));
    }

    println!();
}
