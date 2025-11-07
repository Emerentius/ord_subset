// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
// http://opensource.org/licenses/MIT, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ord_subset_trait::*;
use ord_var::*;

/////////////////////////////////////////////////////////////////////
pub trait OrdSubsetIterExt: Iterator //where Self::Item: OrdSubset
{
    /// Consumes the entire iterator to return the maximum element.
    /// Values outside the ordered subset as given by `.is_outside_order()` are ignored.
    ///
    /// Returns the last element if the comparison determines multiple elements to be equally maximum.
    ///
    /// # Example
    ///
    /// ```
    /// use ord_subset::OrdSubsetIterExt;
    ///
    /// let vec = vec![2.0, 3.0, 5.0, std::f64::NAN];
    /// let max = vec.iter().ord_subset_max().unwrap();
    /// assert_eq!(&5.0, max);
    /// ```
    #[inline]
    fn ord_subset_max(self) -> Option<Self::Item>
    where
        Self: Sized,
        Self::Item: OrdSubset,
    {
        self.filter_map(OrdVar::new_checked)
            .max()
            .map(OrdVar::into_inner) // Option<OrdVar<Item>> => Option<Item>
    }

    /// Consumes the entire iterator to return the minimum element.
    /// Values outside the ordered subset as given by `.is_outside_order()` are ignored.
    ///
    /// Returns the first element if the comparison determines multiple elements to be equally minimum.
    ///
    /// # Example
    ///
    /// ```
    /// use ord_subset::OrdSubsetIterExt;
    ///
    /// let vec = vec![2.0, 3.0, 5.0, std::f64::NAN];
    /// let min = vec.iter().ord_subset_min().unwrap();
    /// assert_eq!(&2.0, min);
    /// ```
    #[inline]
    fn ord_subset_min(self) -> Option<Self::Item>
    where
        Self: Sized,
        Self::Item: OrdSubset,
    {
        self.filter_map(OrdVar::new_checked)
            .min()
            .map(OrdVar::into_inner) // Option<OrdVar<Item>> => Option<Item>
    }

    /// Returns the element that gives the minimum value from the specified function.
    /// Values outside the ordered subset as given by `.is_outside_order()` on the mapped value are ignored.
    ///
    /// Returns the first element if the comparison determines multiple elements to be equally minimum.
    ///
    /// # Example
    ///
    /// ```
    /// extern crate ord_subset;
    /// use ord_subset::OrdSubsetIterExt;
    ///
    /// fn main() {
    ///     let vec = vec![2.0, 3.0, 5.0, std::f64::NAN];
    ///     let min_by = vec.iter().ord_subset_min_by_key(|num| num.recip()).unwrap();
    ///     assert_eq!(&5.0, min_by);
    /// }
    /// ```
    #[inline]
    fn ord_subset_min_by_key<F, B>(self, mut f: F) -> Option<Self::Item>
    where
        F: FnMut(&Self::Item) -> B,
        B: OrdSubset,
        Self: Sized,
    {
        // Ok < Err, always
        self.min_by_key(|it| OrdVar::new_checked(f(it)).ok_or(()))
    }

    /// Returns the element that gives the maximum value from the specified function.
    /// Values outside the ordered subset as given by `.is_outside_order()` on the mapped value are ignored.
    ///
    /// Returns the last element if the comparison determines multiple elements to be equally maximum.
    #[inline]
    fn ord_subset_max_by_key<F, B>(self, mut f: F) -> Option<Self::Item>
    where
        F: FnMut(&Self::Item) -> B,
        B: OrdSubset,
        Self: Sized,
    {
        // Some > None, always
        self.max_by_key(|it| OrdVar::new_checked(f(it)))
    }
}

impl<T: ?Sized + Iterator> OrdSubsetIterExt for T {}
