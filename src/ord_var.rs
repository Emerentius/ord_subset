// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
// http://opensource.org/licenses/MIT, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::cmp::Ordering;
use core::fmt::Debug;
use ord_subset_trait::*;
use core::ops::Deref;

/// Wrapper to signal that the contained variables have a total order. It's illegal to compare two `OrdVar`s that are not ordered.
/// For this reason, it's unsafe to create `OrdVar`s without checking. Checked constructors are available for `OrdSubset` types.
///
/// # Panics
///
/// Panics when `a.partial_cmp(b)` returns `None` for two values `a`,`b`.
#[derive(PartialEq, PartialOrd, Clone, Copy, Debug, Hash)]
pub struct OrdVar<T: PartialOrd + PartialEq>(T);

impl<T: PartialOrd + PartialEq> OrdVar<T> {

	/// Construct an ```OrdVar``` out of the argument.
	///
	/// # Panics
	///
	/// Panics if the argument is outside of the total order.
	#[inline]
	pub fn new(data: T)	-> OrdVar<T>
		where T: Debug + OrdSubset
	{
		if data.is_outside_order() { panic!("Attempted saving data outside of total order into OrdVar: {:?}", data) };
		OrdVar(data)
	}

	/// Constructs an ```Option<OrdVar>``` out of the argument. Returns None if the argument is outside the total order.
	#[inline]
	pub fn new_checked(data: T)	-> Option<OrdVar<T>>
		where T: OrdSubset,
	{
		match data.is_outside_order() {
			true  => None,
			false => Some(OrdVar(data)),
		}
	}

	/// Constructs an `OrdVar` without validity check. Incorrectly constructed `OrdVar`s may panic on calls to `.cmp()`.
	/// The comparison operators (`>`, `>=`, `=`, `!=`, `<`, `<=`) will not panic but may result in surprising behaviour.
	#[inline(always)]
	pub fn new_unchecked(data: T) -> OrdVar<T> {
		OrdVar(data)
	}

	#[inline(always)]
	pub fn into_inner(self) -> T {
		self.0
	}
}

impl<T: PartialOrd + PartialEq> Eq for OrdVar<T> {}

impl<T: PartialOrd + PartialEq> Ord for OrdVar<T> {
	#[inline]
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).expect("OrdVar contains value outside total order")
	}
}

impl<T: PartialOrd + PartialEq> Deref for OrdVar<T> {
	type Target = T;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
