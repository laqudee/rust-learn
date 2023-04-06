use crate::{TwoValuesArray, TwoValueStruct};

pub trait Visitor {
    type Value;

    fn visit_vec(&self, v: Vec<i32>) -> Self::Value;
}

impl Visitor for TwoValueStruct {
    type Value = TwoValueStruct;

    fn visit_vec(&self, v: Vec<i32>) -> Self::Value {
        TwoValueStruct { a: v[0], b: v[1] }
    }
}

impl Visitor for TwoValuesArray {
    type Value = TwoValuesArray;

    fn visit_vec(&self, v: Vec<i32>) -> Self::Value {
        let mut ab = [0i32; 2];

        ab[0] = v[0];
        ab[1] = v[1];

        TwoValuesArray { ab }
    }
}
