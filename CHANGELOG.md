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
