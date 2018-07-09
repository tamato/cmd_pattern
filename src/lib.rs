#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

pub mod base;
pub use base::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

