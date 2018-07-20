// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
// http://opensource.org/licenses/MIT, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ord_subset_trait::*;
use core::cmp::Ordering::{self, Equal, Greater, Less};

static ERROR_BINARY_SEARCH_OUTSIDE_ORDER: &str =
    "Attempted binary search for value outside total order";
static ERROR_BINARY_SEARCH_EXPECT: &str = "Unexpected None for a.partial_cmp(b), a,b inside order. Violated OrdSubset contract or attempted binary search on unsorted data";

// Wrapper for comparison functions
// Treats unordered values as greater than any ordered
#[inline]
fn cmp_unordered_greater_all<T: OrdSubset, F>(a: &T, b: &T, mut compare: F) -> Ordering
where
    F: FnMut(&T, &T) -> Ordering,
{
    match (a.is_outside_order(), b.is_outside_order()) {
        // catch invalids and put them at the end
        // Ordering of two-non-ords in the (true, true) case is irrelevant
        // for the goal of collecting them at the end. However, comparing them
        // as equal will let the algorithm uphold its stability properties
        (true, true) => Equal,
        (true, false) => Greater,
        (false, true) => Less,
        (false, false) => compare(a, b), // the normal case, both valid. Here user function applies.
    }
}

pub trait OrdSubsetSliceExt<T> {
    /// Sort the slice. Values outside the ordered subset are put at the end in their original order.
    ///
    /// This is equivalent to `self.ord_subset_sort_by(|a,b| a.partial_cmp(b).unwrap())`
    ///
    /// # Panics
    ///
    /// Panics when `a.partial_cmp(b)` returns `None` for two values `a`,`b` inside the total order (Violated OrdSubset contract).
    #[cfg(feature = "std")]
    fn ord_subset_sort(&mut self)
    where
        T: OrdSubset;

    /// Sort the slice in reverse order. Values outside the ordered subset are put at the end in their original order (i.e. not reversed).
    ///
    /// # Panics
    ///
    /// Panics when `a.partial_cmp(b)` returns `None` for two values `a`,`b` inside the total order (Violated OrdSubset contract).
    #[cfg(feature = "std")]
    fn ord_subset_sort_rev(&mut self)
    where
        T: OrdSubset;

    /// Sorts the slice, using `compare` to order elements. Values outside the total order are put at the end in their original order.
    /// `compare` will not be called on them. If you wish to handle these yourself, use the regular `.sort_by()`.
    ///
    /// **Warning:** The function interface is identical to the `.sort_by()` interface. Be careful not to miss `ord_subset_` in front.
    /// It would work until you have unordered values in your slice, then crash unexpectedly.
    ///
    /// This delegates to `.sort_by()` in the std library. See [official docs](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by) for
    /// time and space complexity of the current implementation.
    ///
    /// # Panics
    ///
    /// Panics when `a.partial_cmp(b)` returns `None` for two values `a`,`b` inside the total order (Violated OrdSubset contract).
    #[cfg(feature = "std")]
    fn ord_subset_sort_by<F>(&mut self, compare: F)
    where
        T: OrdSubset,
        F: FnMut(&T, &T) -> Ordering;

    /// Sorts the slice, using `key` to extract a key by which to order the sort by. Entries mapping to values outside
    /// the total order will be put at the end in their original order.
    ///
    /// This delegates to `.sort_by()` in the std library. See [official docs](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by) for
    /// time and space complexity of the current implementation.
    #[cfg(feature = "std")]
    fn ord_subset_sort_by_key<B, F>(&mut self, f: F)
    where
        B: OrdSubset,
        F: FnMut(&T) -> B;

    /// Sort the slice. Values outside the ordered subset are put at the end.
    ///
    /// This is equivalent to `self.ord_subset_sort_by(|a,b| a.partial_cmp(b).unwrap())`
    ///
    /// # Panics
    ///
    /// Panics when `a.partial_cmp(b)` returns `None` for two values `a`,`b` inside the total order (Violated OrdSubset contract).
    fn ord_subset_sort_unstable(&mut self)
    where
        T: OrdSubset;

    /// Sort the slice in reverse order. Values outside the ordered subset are put at the end.
    ///
    /// # Panics
    ///
    /// Panics when `a.partial_cmp(b)` returns `None` for two values `a`,`b` inside the total order (Violated OrdSubset contract).
    fn ord_subset_sort_unstable_rev(&mut self)
    where
        T: OrdSubset;

    /// Sorts the slice, using `compare` to order elements. Values outside the total order are put at the end.
    /// `compare` will not be called on them. If you wish to handle these yourself, use the regular `.sort_unstable_by()`.
    ///
    /// **Warning:** The function interface is identical to the `.sort_unstable_by()` interface. Be careful not to miss `ord_subset_` in front.
    /// It would work until you have unordered values in your slice, then crash unexpectedly.
    ///
    /// This delegates to `.sort_by_unstable()` in the std library. See [official docs](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by_unstable) for
    /// time and space complexity of the current implementation.
    ///
    /// # Panics
    ///
    /// Panics when `a.partial_cmp(b)` returns `None` for two values `a`,`b` inside the total order (Violated OrdSubset contract).
    fn ord_subset_sort_unstable_by<F>(&mut self, compare: F)
    where
        T: OrdSubset,
        F: FnMut(&T, &T) -> Ordering;

    /// Sorts the slice, using `key` to extract a key by which to order the sort by. Entries mapping to values outside
    /// the total order will be put at the end.
    ///
    /// This delegates to `.sort_by_unstable()` in the std library. See [official docs](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by_unstable) for
    /// time and space complexity of the current implementation.
    fn ord_subset_sort_unstable_by_key<B, F>(&mut self, f: F)
    where
        B: OrdSubset,
        F: FnMut(&T) -> B;

    /// Binary search a sorted slice for a given element. Values outside the ordered subset need to be at the end of the slice.
    ///
    /// If the value is found then Ok is returned, containing the index of the matching element; if the value is not found then Err is returned, containing the index where a matching element could be inserted while maintaining sorted order.
    ///
    /// # Example
    ///
    /// Looks up a series of five elements. The first is found, with a uniquely determined position; the second and third are not found; the fourth could match any position in `[1,4]`.
    ///
    /// ```
    /// use ord_subset::OrdSubsetSliceExt;
    /// use std::f64;
    ///
    /// let s = [0., 1., 1., 1., 1., 2., 3., 5., 8., 13., 21., 34., 55., f64::NAN, f64::NAN];
    ///
    /// assert_eq!(s.ord_subset_binary_search(&13.),  Ok(9));
    /// assert_eq!(s.ord_subset_binary_search(&4.),   Err(7));
    /// assert_eq!(s.ord_subset_binary_search(&100.), Err(13));
    /// let r = s.ord_subset_binary_search(&1.);
    /// assert!(match r { Ok(1...4) => true, _ => false, });
    ///	assert_eq!(s.ord_subset_binary_search(&f64::INFINITY), Err(13));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the argument is outside of the total order. Also panics when `a.partial_cmp(b)` returns `None` for two values `a`,`b` inside the total order (Violated OrdSubset contract).
    fn ord_subset_binary_search(&self, x: &T) -> Result<usize, usize>
    where
        T: OrdSubset;

    /// Binary search a sorted slice with a comparator function.
    ///
    /// The comparator function should implement an order consistent with the sort order of the underlying slice, returning an order code that indicates whether its argument is Less, Equal or Greater the desired target. The comparator will only be called for values inside the total order.
    ///
    /// It's imperative, that the comparator function doesn't compare its arguments with values outside the total order. This will result in bogus output which cannot be caught by this function.
    ///
    /// If a matching value is found then returns Ok, containing the index for the matched element; if no match is found then Err is returned, containing the index where a matching element could be inserted while maintaining sorted order.
    fn ord_subset_binary_search_by<F>(&self, f: F) -> Result<usize, usize>
    where
        T: OrdSubset,
        F: FnMut(&T) -> Ordering;

    /// Binary search a sorted slice with a key extraction function.
    ///
    /// Assumes that the slice is sorted by the key, for instance with `ord_subset_sort_by_key` using the same key extraction function.
    ///
    /// If a matching value is found then returns `Ok`, containing the index for the matched element; if no match is found then `Err` is returned, containing the index where a matching element could be inserted while maintaining sorted order.
    fn ord_subset_binary_search_by_key<B, F>(&self, b: &B, f: F) -> Result<usize, usize>
    where
        B: OrdSubset,
        F: FnMut(&T) -> B;

    /// Binary search a slice sorted in reverse order for a given element. Values outside the ordered subset need to be at the end of the slice.
    ///
    /// If a matching value is found then returns Ok, containing the index for the matched element; if no match is found then Err is returned, containing the index where a matching element could be inserted while maintaining sorted order.
    ///
    /// # Panics
    ///
    /// Panics if the argument is outside of the total order. Also panics when `a.partial_cmp(b)` returns `None` for two values `a`,`b` inside the total order (Violated OrdSubset contract).
    fn ord_subset_binary_search_rev(&self, x: &T) -> Result<usize, usize>
    where
        T: OrdSubset;
}

impl<T> OrdSubsetSliceExt<T> for [T]
{
    #[cfg(feature = "std")]
    #[inline]
    fn ord_subset_sort(&mut self)
    where
        T: OrdSubset,
    {
        self.as_mut().ord_subset_sort_by(|a, b| a.cmp_unwrap(b))
    }

    #[cfg(feature = "std")]
    #[inline]
    fn ord_subset_sort_by<F>(&mut self, mut compare: F)
    where
        T: OrdSubset,
        F: FnMut(&T, &T) -> Ordering,
    {
        self.as_mut()
            .sort_by(|a, b| cmp_unordered_greater_all(a, b, &mut compare))
    }

    #[cfg(feature = "std")]
    #[inline]
    fn ord_subset_sort_rev(&mut self)
    where
        T: OrdSubset,
    {
        self.as_mut().ord_subset_sort_by(|a, b| b.cmp_unwrap(a))
    }

    #[cfg(feature = "std")]
    #[inline]
    fn ord_subset_sort_by_key<B, F>(&mut self, mut f: F)
    where
        B: OrdSubset,
        F: FnMut(&T) -> B,
    {
        self.as_mut()
            .sort_by(|a, b| cmp_unordered_greater_all(&(f(a)), &(f(b)), CmpUnwrap::cmp_unwrap))
    }

    #[inline]
    fn ord_subset_sort_unstable(&mut self)
    where
        T: OrdSubset,
    {
        self.as_mut()
            .ord_subset_sort_unstable_by(|a, b| a.cmp_unwrap(b))
    }

    #[inline]
    fn ord_subset_sort_unstable_by<F>(&mut self, mut compare: F)
    where
        T: OrdSubset,
        F: FnMut(&T, &T) -> Ordering,
    {
        self.as_mut()
            .sort_unstable_by(|a, b| cmp_unordered_greater_all(a, b, &mut compare))
    }

    #[inline]
    fn ord_subset_sort_unstable_rev(&mut self)
    where
        T: OrdSubset,
    {
        self.as_mut()
            .ord_subset_sort_unstable_by(|a, b| b.cmp_unwrap(a))
    }

    #[inline]
    fn ord_subset_sort_unstable_by_key<B, F>(&mut self, mut f: F)
    where
        B: OrdSubset,
        F: FnMut(&T) -> B,
    {
        self.as_mut().sort_unstable_by(|a, b| {
            cmp_unordered_greater_all(&(f(a)), &(f(b)), CmpUnwrap::cmp_unwrap)
        })
    }

    #[inline]
    fn ord_subset_binary_search(&self, x: &T) -> Result<usize, usize>
    where
        T: OrdSubset,
    {
        if x.is_outside_order() {
            panic!(ERROR_BINARY_SEARCH_OUTSIDE_ORDER)
        };
        self.ord_subset_binary_search_by(|other| {
            other.partial_cmp(x).expect(ERROR_BINARY_SEARCH_EXPECT)
        })
    }

    #[inline]
    fn ord_subset_binary_search_by<F>(&self, mut f: F) -> Result<usize, usize>
    where
        T: OrdSubset,
        F: FnMut(&T) -> Ordering,
    {
        self.as_ref().binary_search_by(|other| {
            match other.is_outside_order() {
                true => Greater, // unordered always at end
                false => f(other),
            }
        })
    }

    #[inline]
    fn ord_subset_binary_search_by_key<B, F>(&self, b: &B, mut f: F) -> Result<usize, usize>
    where
        B: OrdSubset,
        F: FnMut(&T) -> B,
    {
        if b.is_outside_order() {
            panic!(ERROR_BINARY_SEARCH_OUTSIDE_ORDER)
        };
        // compare ordered values as expected
        // wrap it in a function that deals with unordered, so this one never sees them
        let cmp_ord = |a: &B, b: &B| a.partial_cmp(b).expect(ERROR_BINARY_SEARCH_EXPECT);
        self.as_ref()
            .binary_search_by(|k| cmp_unordered_greater_all(&f(k), b, &cmp_ord))
    }

    #[inline]
    fn ord_subset_binary_search_rev(&self, x: &T) -> Result<usize, usize>
    where
        T: OrdSubset,
    {
        if x.is_outside_order() {
            panic!(ERROR_BINARY_SEARCH_OUTSIDE_ORDER)
        };
        self.ord_subset_binary_search_by(|other| {
            x.partial_cmp(other).expect(ERROR_BINARY_SEARCH_EXPECT)
        })
    }
}
