// 模板方法模式

mod template_method;

use template_method::{client_code, ConcreteStruct1, ConcreteStruct2};

fn main() {
    println!("Same client code can work with different concrete implementations:");
    client_code(ConcreteStruct1);
    println!();

    println!("Same client code can work with different concrete implementations:");
    client_code(ConcreteStruct2);
}
