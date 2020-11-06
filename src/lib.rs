#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![feature(min_const_generics)]
#![feature(test)]
// #![warn(missing_docs)]
// #![warn(missing_doc_code_examples)]

extern crate test;

pub mod ch1;
pub mod ch2;
pub mod ch3;
pub mod ch4;
pub mod ch5;
pub mod vec;

pub use ch1::*;
pub use ch2::*;
pub use ch3::*;
pub use ch4::*;
pub use ch5::*;
pub use vec::*;
