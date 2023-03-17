use crate::List::{Cons, Nil};
use std::ops::Deref;

fn main() {
    let b = Box::new(5);
    println!("b = {} ", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(2, Box::new(Nil))))));

    println!("{:?}", list); 


}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_test() {
        let x = 5;
        let y1 = &x;
        let y2 = Box::new(x);
        let y3 = MyBox::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y1);
        assert_eq!(5, *y2);
        assert_eq!(5, *y3); // 底层运行的是*(y.deref())
    }
}
