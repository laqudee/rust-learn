mod users;

use crate::users::UserCollection;

fn main() {
    // Standard Iterator
    let array = &[1, 2, 3];
    let iterator = array.iter();
    iterator.for_each(|e| print!("{}, ", e));

    println!("\n\nLet's test our own iterator.\n");

    // Custom Iterator
    let users = UserCollection::new();
    let mut iterator = users.iter();

    println!("1nd element: {:?}", iterator.next());
    println!("2nd element: {:?}", iterator.next());
    println!("3rd element: {:?}", iterator.next());
    println!("4th element: {:?}", iterator.next());

    print!("\nAll elements in user collection: ");
    users.iter().for_each(|e| print!("{} ", e));

    println!();
}
