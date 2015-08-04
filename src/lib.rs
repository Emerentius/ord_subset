#![feature(iter_cmp)]
#![feature(iter_min_max)]
//! Ever wanted to call `.max()` on an iterator of floats? Now you can! Well, almost: `.partial_max()`.
//!
//! This crate is for types like the float primitive types `f32` and `f64`: Types that are totally ordered *except for these particular values*.
//!
//! I call these types almost ordered. They can be marked with the `AlmostOrd` trait that this crate defines.
//! Such types can be put in the `OrdVar` struct. Wrapping your value in this marks to other code that the contents are ordered, thus fulfilling generic `Ord` trait bounds.
//!
//! For convenience, iterators and slices are extended so that AlmostOrd types have access to methods equivalent to `.max()` and `.sort()`.
//! Values in the unordered subset of a type that is AlmostOrd are handled in a consistent manner (Ignored or put at the end).
//!
//! # Stability
//!
//! Internally, this crate uses the same methods that it extends to `AlmostOrd` types. Unstable traits in `std` are therefore also unstable here. It will stay at `0.x.y` until everything has stabilised. Breaking changes will increment the minor version.
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! almost_ord = "~0.1.0"
//! ```
//!
//!
//! ```
//! extern crate almost_ord;
//! use almost_ord::{AlmostOrdIterExt, AlmostOrdSliceExt};
//!
//! fn main() {
//! 	// Slices. Works on vector, too.
//! 	let mut s = [5.0, std::f64::NAN, 3.0, 2.0];
//! 	s.partial_sort();
//! 	assert_eq!(&s[0..3], &[2.0, 3.0, 5.0]);
//! 	assert_eq!(s.partial_binary_search(&5.0), Ok(2));
//!
//! 	// iterators
//! 	assert_eq!( s.iter().partial_max(), Some(&5.0) );
//! 	assert_eq!( s.iter().partial_min(), Some(&2.0) );
//! }
//! ```
//!
//! # License
//! Licensed under the Apache License, Version 2.0
//! http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
//! http://opensource.org/licenses/MIT, at your
//! option. This file may not be copied, modified, or distributed
//! except according to those terms.
mod iter_ext;
mod ord_var;
mod rev_option;
mod slice_ext;
mod almost_ord_trait;

pub use iter_ext::*;
pub use ord_var::*;
pub use slice_ext::*;
pub use almost_ord_trait::*;
