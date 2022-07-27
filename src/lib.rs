#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]
// #![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

mod sensors;
mod traits;

pub use crate::sensors::*;
pub use crate::traits::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
