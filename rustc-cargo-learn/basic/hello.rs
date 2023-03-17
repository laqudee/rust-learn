fn main() {
    println!("Hello, world!");
    foo::hello();
}

mod foo {
    pub fn hello() {
        println!("hello, foo!");
    }
}