fn arry_map_play() {
    let x = [1, 2, 3, 4];
    let y = &x.map(|v| v * 2);
    println!("y: {:?}", y);

    let mut temp = 0;
    let y2 = x.map(|v| {
        temp += 1;
        v * temp
    });
    println!("y2: {:?}", y2);

    let x2 = ["Ferris", "Buerr", "Day", "odd"];
    let y3 = x2.map(|v| v.len());
    println!("y3: {:?}", y3);
}

// fn array_try_map_play() {
//     let a = ["1", "2", "3", "4"];
//     let b = a.try_map(|v| v.parse::<u32>()).unwrap().map(|v| v + 1);
//     println!("b: {:?}", b);

//     let a1 = ["1", "2a", "3"];
//     let b1 = a1.try_map(|v| v.parse::<u32>());
//     assert!(b1.is_err());

//     use std::num::NonZeroU32;
//     let z = [1, 2, 0, 3, 4];
//     assert_eq!(z.try_map(NonZeroU32::new), None);
//     let a = [1, 2, 3];
//     let b = a.try_map(NonZeroU32::new);
//     let c = b.map(|x| x.map(NonZeroU32::get));
//     assert_eq!(c, Some(a));
// }

// fn array_zip_play() {
//     let x = [1, 2, 3];
//     let y = [6; 3];
//     let z = x.zip(y);
//     assert_eq!(z, [(1, 6), (2, 6), (3, 6)]);
// }

pub fn play() {
    arry_map_play();
    // println!("+++++++++++++Arry Try Map Function+++++++++++++");
    // array_try_map_play();
}
