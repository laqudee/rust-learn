//! # My Crate
//! 
//! `my_crate` is  a collection of utilities to make performing certain
//! calculations more convenient.

/// Add Function
/// 
/// #Example
/// 
/// ```
/// let a = 2;
/// let b = 2;
/// let result = my_crate::add(a, b);
/// 
/// assert_eq!(4, result);
/// ```


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
