fn common_way_create_raw_pointers() {
    let my_num: i32 = 10;
    let my_num_ptr: *const i32 = &my_num;
    let mut my_speed = 88;
    let my_speed_ptr: *mut i32 = &mut my_speed;

    println!(
        "my_num_ptr: {:?}; my_speed_ptr: {:?}",
        my_num_ptr, my_speed_ptr
    );
    // my_num_ptr: 0xa8fa1c; my_speed_ptr: 0xa8fa2c

    // to get a pointer to a boxed value, dereference the box
    let my_num_box = Box::new(10);
    let my_num_box_ptr: *const i32 = &*my_num_box;
    let mut my_speed_box = Box::new(99);
    let my_speed_box_ptr: *mut i32 = &mut *my_speed_box;
    println!(
        "my_num_box_ptr: {:?}; my_speed_box_ptr: {:?}",
        my_num_box_ptr, my_speed_box_ptr
    );
    // my_num_box_ptr: 0xb57840; my_speed_box_ptr: 0xb57860
}

fn consume_a_box() {
    let my_speed = Box::new(88);
    let my_speed = Box::into_raw(my_speed);
    println!("my_speed: {:?}", my_speed);
    // my_speed: 0xe4830

    unsafe {
        drop(Box::from_raw(my_speed));
    }
}

fn create_pointer_using_ptr_addr_of() {
    #[derive(Debug, Default, Copy, Clone)]
    #[repr(C, packed)]
    struct S {
        aligned: u8,
        unaligned: u32,
    }

    let s = S::default();
    let p = std::ptr::addr_of!(s.unaligned);
    let p2 = std::ptr::addr_of!(s.aligned);
    println!("p: {:?}; p2: {:?}", p, p2);
}

extern crate libc;
use std::mem;
fn get_it_from_c() {
    unsafe {
        let my_num = libc::malloc(mem::size_of::<i32>()) as *mut i32;
        println!("my_num: {:?}", my_num);
        if my_num.is_null() {
            panic!("failed to allocate memory");
        }
        libc::free(my_num as *mut libc::c_void);
    }
}

// use std::ptr;
// fn implementations_experimental_play() {
//     let slice: *const [i8] = ptr::slice_from_raw_parts(ptr::null(), 3);
//     assert_eq!(slice.len(), 3);
//     assert_eq!(slice.as_ptr(), ptr::null());

//     let x = &[1, 2, 4] as *const [i32];

//     unsafe {
//         assert_eq!(x.get_unchecked(1), x.as_ptr().ass(1));
//     }
// }

fn impl_const_t() {
    let s = "Follow thew rabbit";
    let ptr = s.as_ptr();
    println!("ptr is null: {}", ptr.is_null());
}

fn as_ref_play() {
    let ptr = &10u8 as *const u8;
    println!("ptr: {:?}", ptr);
    unsafe {
        if let Some(val_back) = ptr.as_ref() {
            println!("We got back the value: {val_back}!");
        }

        let val_back = &*ptr;
        println!("We got back the value: {val_back}!");
    }

    let s = "123";
    let ptr = s.as_ptr();
    unsafe {
        println!("{}", *ptr.offset(1) as char);
        println!("{}", *ptr.offset(2) as char);
    }

    let data = [1u8, 2, 3, 4, 5];
    let mut ptr = data.as_ptr();
    let step = 2;
    let end_rounded_up = ptr.wrapping_offset(6);
    println!("end_rounded_up: {:?}", end_rounded_up);

    // this loop prints "1, 3, 5";
    while ptr != end_rounded_up {
        unsafe {
            print!("{}, ", *ptr);
        }
        ptr = ptr.wrapping_offset(step);
    }
    println!("+++++++++++offset_from++++++++++");

    let a = [0; 5];
    let ptr1: *const i32 = &a[1];
    let ptr2: *const i32 = &a[3];
    println!("ptr1: {:?}", ptr1);
    unsafe {
        println!("{}", ptr2.offset_from(ptr1));
        println!("{}", ptr1.offset_from(ptr2));
        println!("{:?}, {:?}", ptr1.offset(2), ptr2.offset(-2));
    }

    let s = "123";
    let ptr = s.as_ptr();
    unsafe {
        println!("{}", *ptr.add(1) as char);
        println!("{}", *ptr.add(2) as char);
    }

    let data = [1u8, 2, 3, 4, 5];
    let mut ptr = data.as_ptr();
    let step = 2;
    let end_rounded_up = ptr.wrapping_add(6);

    while ptr != end_rounded_up {
        unsafe {
            print!("{}, ", *ptr);
        }
        ptr = ptr.wrapping_add(step);
    }
}

fn impl_mut_t() {
    let mut s = [1, 2, 3];
    let ptr = s.as_mut_ptr();
    println!("ptr is null {}", ptr.is_null());
}

pub fn play() {
    common_way_create_raw_pointers();
    consume_a_box();
    create_pointer_using_ptr_addr_of();
    get_it_from_c();
    impl_const_t();
    as_ref_play();
    impl_mut_t();
}
