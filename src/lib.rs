//! Ever wanted to call `.max()` on an iterator of floats? Now you can! Well, almost: `.ord_subset_max()`.
//!
//! This crate is for types like the float primitive types `f32` and `f64`: Types that are totally ordered *except for these particular values*.
//!
//! I call these types subset-ordered. They can be marked with the `OrdSubset` trait that this crate defines.
//! Such types can be put in the `OrdVar` struct. Wrapping your value in this marks to other code that the contents are ordered, thus fulfilling generic `Ord` trait bounds.
//!
//! For convenience, iterators and slices are extended so that `OrdSubset` types have access to methods equivalent to `.max()` and `.sort()`.
//! Values in the unordered subset of a type that is `OrdSubset` are handled in a consistent manner (Ignored or put at the end).
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! ord_subset = "3"
//! ```
//!
//!
//! ```
//! use ord_subset::{OrdSubsetIterExt, OrdSubsetSliceExt};
//!
//! // Slices. Works on vector, too.
//! let mut s = [5.0, std::f64::NAN, 3.0, 2.0];
//! s.ord_subset_sort_unstable();
//! assert_eq!(&s[0..3], &[2.0, 3.0, 5.0]);
//! assert_eq!(s.ord_subset_binary_search(&5.0), Ok(2));
//!
//! // iterators
//! assert_eq!( s.iter().ord_subset_max(), Some(&5.0) );
//! assert_eq!( s.iter().ord_subset_min(), Some(&2.0) );
//! ```
//!
//! # License
//! Licensed under the Apache License, Version 2.0
//! <https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//! <https://opensource.org/licenses/MIT>, at your
//! option. This file may not be copied, modified, or distributed
//! except according to those terms.
//#![cfg_attr(feature="unstable", unstable)]
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "std")] // attribute not necessary, but rls warns without
extern crate core;

mod iter_ext;
mod ord_subset_trait;
mod ord_var;
mod slice_ext;

pub use iter_ext::*;
pub use ord_subset_trait::*;
pub use ord_var::*;
pub use slice_ext::*;
