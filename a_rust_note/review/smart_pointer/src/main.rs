use std::{ops::Deref, rc::Rc};
// use std::ops::Drop; // 包含在prelude中无需导入
pub mod libs;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum ListRc {
    ConsRc(i32, Rc<ListRc>),
    NilRc,
}

use crate::List::{Cons, Nil};
use crate::ListRc::{ConsRc, NilRc};

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data  `{}`", self.data);
    }
}

fn main() {
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    drop(c); // 析构函数，用来清理实例。 drop()在prelude中，不需要导入
    println!("CustomSmartPointers created.");

    let a = Rc::new(ConsRc(5, Rc::new(ConsRc(6, Rc::new(ConsRc(7, Rc::new(NilRc)))))));
    println!("count: {}", Rc::strong_count(&a));
    let b = ConsRc(3, Rc::clone(&a));
    println!("count: {}", Rc::strong_count(&a));
    let c = ConsRc(4, Rc::clone(&a));
    println!("count: {}", Rc::strong_count(&a));

    libs::it_work();
}

fn hello(name: &str) {
    println!("Hello, {name}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_box_work() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, *y);
    }

    #[test]
    fn it_work() {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
}
