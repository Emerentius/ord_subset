Version 1.1.0 (2016-01-25)
==========================

* Implement `Hash` for `OrdVar(T)`, if `T: Hash`

Version 1.2.0 (2016-03-08)
==========================

* Add two new default methods `ord_subset_min_by_key` and `ord_subset_max_by_key` for `OrdSubsetIterExt` to mimic std library.

Version 2.0.0 (2017-02-28)
==========================
* Allow `*_by_key()`  functions for iterators and slices when the items aren't `OrdSubset`. Only the keys need to be. This changes the extension traits' bounds, therefore technically a [breaking change] (cause for major version bump)
* Add long overdue `ord_subset_sort_by_key()` and `ord_subset_binary_search_by_key()` for slices

Version 2.1.0 (2017-09-23)
==========================
* `OrdVar::new_unchecked()` is no longer marked unsafe. Incorrectly constructed `OrdVar`s can cause crashes and surprising behaviour
but no memory unsafety
* In slice sorting, uphold sort stability for values outside total order.

Version 3.0.0 (2017-10-21)
==========================
* Add `sort_unstable*` variants, following the std library additions in 1.20. Technically a [breaking change].
* Generalized traits so that `.ord_subset_binary_search*` works on immutable slices and anything else that implements AsRef<[T]>, but not AsMut<[T]>. Fixing this major oversight is also technically a [breaking change].
* `#![no_std]` support with new opt-out feature `std`
  Stable sorts are unavailable in `no_std` mode.
  OrdSubset implementations for f32, f64 with `no_std` require the `unstable` feature of this crate (impls readded in 3.1 without `unstable`).
* Stabilized `_rev()` variants

Unreleased
==========================
* Overload ops for `OrdVar` to work on anything which works for the contained type, checking each result for order. Added by new default feature `ops` with opt-in feature `unchecked_ops`.
* Add `OrdSubset` impls for `f32`, `f64` in `no_std` mode without requiring `unstable`.
* Add `OrdSubset` impls for all `Ord` intrinsics