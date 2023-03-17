fn basic_play() {
    // slicing a Vec
    let vec = vec![1, 2, 3];
    let _int_slice = &vec[..];
    // coercing an array to a slice
    let _str_slice: &[&str] = &["one", "two", "three"];

    let mut x = [1, 2, 3];
    let x = &mut x[..];
    x[1] = 7;
    assert_eq!(x, &[1, 7, 3]);
    println!("x: {:?}", x);
}

fn slice_iterator_play() {
    let numbers = &[0, 1, 2];
    for n in numbers {
        println!("{n} is number!");
    }

    let scores = &mut [1, 8, 9];
    for score in scores.into_iter() {
        *score += 10;
    }
    println!("{:?}", scores);
}

// fn impl_box_play() {
//     let mut values = Box::<[u32]>::new_uninit_slicce(3);

//     let values = unsafe {
//         values[0].as_mut_ptr().write(1);
//         values[1].as_mut_ptr().write(2);
//         values[2].as_mut_ptr().write(3);

//         values.assume_init()
//     };

//     assert_eq!(*values, [1, 2, 3])
// }

fn impl_t_play() {
    let a = [1, 2, 3];
    println!("length of a is: {}", a.len());

    println!("a is empty: {}", a.is_empty());

    let a_first = a.first();
    let w: &[i32] = &[];
    println!("a_first: {:?}; the first of w is: {:?}", a_first, w.first());

    let x = &mut [1, 2, 3];
    if let Some(first) = x.first_mut() {
        *first = 5;
    }
    println!("x: {:?}", x);

    {
        let x = &[0, 1, 2];
        if let Some((first, elements)) = x.split_first() {
            println!("first element: {}; others elements: {:?}", first, elements);
        }

        let x = &mut [0, 1, 2];
        if let Some((first, elements)) = x.split_first_mut() {
            *first = 7;
            elements[0] = 4;
            elements[1] = 5;
        }
        println!("x: {:?}", x);
    }

    {
        let x = &[0, 1, 2];
        if let Some((last, elements)) = x.split_last() {
            println!("elements: {:?}; last: {}", elements, last);
        }
        let x = &mut [0, 1, 2];
        if let Some((last, elements)) = x.split_last_mut() {
            elements[0] = 10;
            elements[1] = 9;
            *last = 8;
        }
        println!("x: {:?}", x);
    }
}

fn get_get_mut() {
    let v = &[10, 40, 30];
    println!("get index = 1 is {:?}", v.get(1)); // Some(40)
    println!("get index of 0..2 is {:?}", v.get(0..2)); // Some([10, 40])
    println!("get index = 3 is {:?}", v.get(3)); // None
    println!("get index  of 0..4 is {:?}", v.get(0..4)); // None

    let x = &mut [0, 1, 2];
    if let Some(elem) = x.get_mut(1) {
        *elem = 42;
    }
    assert_eq!(x, &[0, 42, 2]);

    unsafe {
        assert_eq!(x.get_unchecked(1), &42);
    }

    let x = &mut [1, 2, 4];
    unsafe {
        let element = x.get_unchecked_mut(1);
        *element = 13;
    }
    println!("x: {:?}", x);
}

fn as_ptr_or_other() {
    let x = &[1, 2, 4];
    let x_ptr = x.as_ptr();
    unsafe {
        for i in 0..x.len() {
            println!(
                "x.get_unchecked({}) is {:?}; &*x_ptr.add({}) is {:?}",
                i,
                x.get_unchecked(i),
                i,
                &*x_ptr.add(i)
            );
        }
    }

    let x = &mut [1, 2, 4];
    let x_ptr = x.as_mut_ptr();
    unsafe {
        for i in 0..x.len() {
            *x_ptr.add(i) += 2;
        }

        assert_eq!(x, &[3, 4, 6]);
    }
}

pub fn play() {
    basic_play();
    slice_iterator_play();
    impl_t_play();
    get_get_mut();
    as_ptr_or_other();
}
