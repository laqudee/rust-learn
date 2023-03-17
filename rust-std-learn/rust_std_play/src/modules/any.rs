use std::any::{Any, TypeId};
// use std::any::{Provider, Demand, request_ref};
use std::fmt::Debug;

pub fn play() {
    let boxed: Box<dyn Any> = Box::new(3_i32);

    // You're more likely to want this:
    let actual_id = (&*boxed).type_id();
    // ... than this:
    let boxed_id = boxed.type_id();

    assert_eq!(actual_id, TypeId::of::<i32>());
    assert_eq!(boxed_id, TypeId::of::<Box<dyn Any>>());

    println!("actual_id: {:?}; boxed_id: {:?}", actual_id, boxed_id);

    let my_string = "Hello World".to_string();
    do_work(&my_string);

    let my_i8: i8 = 100;
    do_work(&my_i8);
}

fn log<T: Any + Debug>(value: &T) {
    let value_any = value as &dyn Any;

    // Try to convert our value to a String. If successful, we want to
    // output the String 's length as well as its value. If not, it's a
    // different type: just print it out unadorned
    match value_any.downcast_ref::<String>() {
        Some(as_string) => {
            println!("String ({}) : {}", as_string.len(), as_string);
        }
        None => {
            println!("{value:?}");
        }
    }
}

fn do_work<T: Any + Debug>(value: &T) {
    log(value);
    // ...do some other work
}

// trait MyTrait: Provider {

// }

// impl dyn MyTrait + '_ {
//     pub fn get_context_by_ref<T: ?Sized + 'static>(&self) -> Option<&T> {
//         request_ref::<T>(self)
//     }
// }

// impl MyTrait for SomeConcreteType {

// }

// impl Provider for SomeConcreteType {
//     fn provide<'a>(&'a self, demand: &mut Demand<'a>) {
//         demand.provide_ref::<String>(&self.some_string);
//     }
// }

// fn use_my_trait(obj: &dyn MyTrait) {
//     let _ = obj.get_context_by_ref::<String>().unwrap();
// }