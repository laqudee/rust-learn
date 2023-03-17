use std::cell::RefCell;
// use std::cell::Cell;
// use std::ptr::NonNull;
// use std::process::abort;
// use std::marker::PhantomData;
use std::collections::HashMap;
use std::rc::Rc;

pub fn play() {
    mutability_inside_of_something_immutable();
    implementation_detail_of_logically_immutable();
}

fn mutability_inside_of_something_immutable() {
    let shared_map = Rc::new(RefCell::new(HashMap::new()));
    // create a new block to limit the scope of the dynamnic borrow
    {
        let mut map = shared_map.borrow_mut();
        map.insert("africa", 92388);
        map.insert("kyoto", 114678);
        map.insert("piccadilly", 118902);
        map.insert("marbles", 38);
    }

    let total: i32 = shared_map.borrow().values().sum();

    println!("{total}");
}

#[derive(Debug)]
struct Graph {
    edges: Vec<(i32, i32)>,
    span_tree_cache: RefCell<Option<Vec<(i32, i32)>>>,
}

impl Graph {
    fn minimum_spanning_tree(&self) -> Vec<(i32, i32)> {
        println!("self: {:?}; {:?}", self.edges, self.span_tree_cache);
        self.span_tree_cache
            .borrow_mut()
            .get_or_insert_with(|| self.calc_span_tree())
            .clone()
    }

    fn calc_span_tree(&self) -> Vec<(i32, i32)> {
        vec![]
    }
}

fn implementation_detail_of_logically_immutable() {
    let graph = Graph {
        edges: vec![(1, 2), (3, 4)],
        span_tree_cache: RefCell::new(Some(vec![(1, 2), (3, 4), (5, 6)])),
    };
    let result = graph.minimum_spanning_tree();
    println!("result: {:?}", result);
}

// struct Rc<T: ?Sized> {
//     ptr: NonNull<RcBox<T>>,
//     phantom: PhantomData<RcBox<T>>,
// }

// struct RcBox<T: ?Sized> {
//     strong: Cell<usize>,
//     refcount: Cell<usize>,
//     value: T,
// }

// impl<T: ?Sized> Clone for Rc<T> {
//     fn clone(&self) -> Self {
//         self.inc_strong();
//         Rc {
//             ptr: self.ptr,
//             phantom: PhantomData,
//         }
//     }
// }

// trait RcBoxPtr<T: ?Sized> {
//     fn inner(&self) -> &RcBox<T>;

//     fn strong(&self) -> usize {
//         self.inner().strong.get()
//     }

//     fn inc_strong(&self) {
//         self.inner()
//             .strong
//             .set(self.strong()
//                      .checked_add(1)
//                      .unwrap_or_else(|| abort() ));
//     }
// }

// impl<T: ?Sized> RcBoxPtr<T> for Rc<T> {
//     fn inner(&self) -> &RcBox<T> {
//         unsafe {
//             self.ptr.as_ref();
//         }
//     }
// }