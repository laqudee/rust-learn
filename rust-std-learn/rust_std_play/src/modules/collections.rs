// Iterator

fn iterator_play() {
    let vec = vec![1, 2, 3, 4];
    for x in vec.iter() {
        println!("vec contained: {x:?}");
    }

    let mut vec1 = vec![1, 2, 3, 4];
    for x in vec1.iter_mut() {
        *x *= 2;
    }
    println!("vec1: {:?}", vec1);

    vec1.extend(vec);
    println!("vec1: {:?}", vec1);

    iterator_others_play();
}

use std::collections::VecDeque;
fn iterator_others_play() {
    let vec = vec![1, 2, 3, 4];
    let buf: VecDeque<_> = vec.into_iter().collect();
    println!("buf: {:?}", buf);
    let vec = vec![3, 6, 9];
    let result: Vec<i32> = vec.iter().map(|v| v * v).collect();
    println!("result: {:?}", result);

    for x in vec.iter().rev() {
        println!("vec contained: {x:?}");
    }
}

// Entries
use std::collections::btree_map::BTreeMap;
fn enteies_play() {
    let mut count = BTreeMap::new();
    let message = "she sells sea shells by the sea shore";

    for c in message.chars() {
        *count.entry(c).or_insert(0) += 1;
    }

    assert_eq!(count.get(&'s'), Some(&8));
    println!("Number of occurrences of each character");

    for (char, count) in &count {
        println!("{char}: {count}");
    }

    entries_others_play();
}

#[derive(Debug)]
struct Person {
    blood_alcohol: f32,
}
fn entries_others_play() {
    let orders = vec![1, 2, 1, 2, 3, 4, 1, 2, 2, 3, 4, 1, 1, 1];

    let mut blood_alcohol = BTreeMap::new();

    for id in orders {
        let person = blood_alcohol
            .entry(id)
            .or_insert(Person { blood_alcohol: 0.0 });

        // println!("person: {person:?}");
        person.blood_alcohol *= 0.9;

        if person.blood_alcohol > 0.3 {
            // Too drunk... for now.
            println!("Sorry {id}, I have to cut you off");
        } else {
            // Have another!
            person.blood_alcohol += 0.1;
        }
    }
}

// Insert and complex keys
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct Foo {
    a: u32,
    b: &'static str,
}

impl PartialEq for Foo {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a
    }
}

impl Eq for Foo {}

impl Hash for Foo {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.a.hash(h);
    }
}

impl PartialOrd for Foo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.a.partial_cmp(&other.a)
    }
}

impl Ord for Foo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.a.cmp(&other.a)
    }
}

fn insert_complex_play() {
    let mut map = BTreeMap::new();

    map.insert(Foo { a: 1, b: "bza" }, 99);
    // We already have a Foo with an a of 1, so this will be updating the value.
    map.insert(Foo { a: 1, b: "xyz" }, 100);

    println!("map: {:?}", map);
    assert_eq!(map.values().next().unwrap(), &100);
    assert_eq!(map.keys().next().unwrap().b, "bza");
}

pub fn play() {
    iterator_play();
    enteies_play();
    insert_complex_play();
}
