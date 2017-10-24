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

impl<T: PartialOrd + PartialEq> AsRef<T> for OrdVar<T> {
	#[inline(always)]
	fn as_ref(&self) -> &T {
		&self.0
	}
}

#[cfg(ops)]
mod ops {
	// would love to be able to macro these away somehow
	use core::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor, Shl, Shr, Neg, Not,
                AddAssign, SubAssign, MulAssign, DivAssign, RemAssign, BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign,};
	use core::fmt::Debug;
	use ord_subset_trait::*;
	use super::OrdVar;

	#[inline(always)]
	fn construct<T: PartialOrd + PartialEq>(t: T) -> OrdVar<T> {
		match cfg!(feature = "unchecked_ops") {
			true => OrdVar::new_unchecked(t),
			false => OrdVar::new(t),
		}
	}
	
	// -----------------  binary ops -----------------------------------------------

	impl<T, RHS> Add<RHS> for OrdVar<T>
		where T: PartialOrd + PartialEq + Add<RHS>,
			T::Output: PartialOrd + PartialEq + Debug + OrdSubset,
	{
		type Output = OrdVar<T::Output>;
		fn add(self, rhs: RHS) -> Self::Output {
			construct(self.into_inner().add(rhs))
		}
	}

	impl<T, RHS> Sub<RHS> for OrdVar<T>
		where T: PartialOrd + PartialEq + Sub<RHS>,
			T::Output: PartialOrd + PartialEq + Debug + OrdSubset,
	{
		type Output = OrdVar<T::Output>;
		fn sub(self, rhs: RHS) -> Self::Output {
			construct(self.into_inner().sub(rhs))
		}
	}

	impl<T, RHS> Mul<RHS> for OrdVar<T>
		where T: PartialOrd + PartialEq + Mul<RHS>,
			T::Output: PartialOrd + PartialEq + Debug + OrdSubset,
	{
		type Output = OrdVar<T::Output>;
		fn mul(self, rhs: RHS) -> Self::Output {
			construct(self.into_inner().mul(rhs))
		}
	}

	impl<T, RHS> Div<RHS> for OrdVar<T>
		where T: PartialOrd + PartialEq + Div<RHS>,
			T::Output: PartialOrd + PartialEq + Debug + OrdSubset,
	{
		type Output = OrdVar<T::Output>;
		fn div(self, rhs: RHS) -> Self::Output {
			construct(self.into_inner().div(rhs))
		}
	}

	impl<T, RHS> Rem<RHS> for OrdVar<T>
		where T: PartialOrd + PartialEq + Rem<RHS>,
			T::Output: PartialOrd + PartialEq + Debug + OrdSubset,
	{
		type Output = OrdVar<T::Output>;
		fn rem(self, rhs: RHS) -> Self::Output {
			construct(self.into_inner().rem(rhs))
		}
	}

	impl<T, RHS> BitAnd<RHS> for OrdVar<T>
		where T: PartialOrd + PartialEq + BitAnd<RHS>,
			T::Output: PartialOrd + PartialEq + Debug + OrdSubset,
	{
		type Output = OrdVar<T::Output>;
		fn bitand(self, rhs: RHS) -> Self::Output {
			construct(self.into_inner().bitand(rhs))
		}
	}

	impl<T, RHS> BitOr<RHS> for OrdVar<T>
		where T: PartialOrd + PartialEq + BitOr<RHS>,
			T::Output: PartialOrd + PartialEq + Debug + OrdSubset,
	{
		type Output = OrdVar<T::Output>;
		fn bitor(self, rhs: RHS) -> Self::Output {
			construct(self.into_inner().bitor(rhs))
		}
	}

	impl<T, RHS> BitXor<RHS> for OrdVar<T>
		where T: PartialOrd + PartialEq + BitXor<RHS>,
			T::Output: PartialOrd + PartialEq + Debug + OrdSubset,
	{
		type Output = OrdVar<T::Output>;
		fn bitxor(self, rhs: RHS) -> Self::Output {
			construct(self.into_inner().bitxor(rhs))
		}
	}

	impl<T, RHS> Shl<RHS> for OrdVar<T>
		where T: PartialOrd + PartialEq + Shl<RHS>,
			T::Output: PartialOrd + PartialEq + Debug + OrdSubset,
	{
		type Output = OrdVar<T::Output>;
		fn shl(self, rhs: RHS) -> Self::Output {
			construct(self.into_inner().shl(rhs))
		}
	}

	impl<T, RHS> Shr<RHS> for OrdVar<T>
		where T: PartialOrd + PartialEq + Shr<RHS>,
			T::Output: PartialOrd + PartialEq + Debug + OrdSubset,
	{
		type Output = OrdVar<T::Output>;
		fn shr(self, rhs: RHS) -> Self::Output {
			construct(self.into_inner().shr(rhs))
		}
	}

	// ------------------------ binary assign ops ----------------------------------

	impl<T, RHS> AddAssign<RHS> for OrdVar<T>
		where T: PartialOrd + PartialEq + AddAssign<RHS> + OrdSubset,
	{
		fn add_assign(&mut self, rhs: RHS) {
			self.0.add_assign(rhs);
			if !cfg!(feature="unchecked_ops") {
				assert!(!self.0.is_outside_order(), "Result of {}= operation is outside order", "+");
			}
		}
	}

	impl<T, RHS> SubAssign<RHS> for OrdVar<T>
		where T: PartialOrd + PartialEq + SubAssign<RHS> + OrdSubset,
	{
		fn sub_assign(&mut self, rhs: RHS) {
			self.0.sub_assign(rhs);
			if !cfg!(feature="unchecked_ops") {
				assert!(!self.0.is_outside_order(), "Result of {}= operation is outside order", "-");
			}
		}
	}

	impl<T, RHS> MulAssign<RHS> for OrdVar<T>
		where T: PartialOrd + PartialEq + MulAssign<RHS> + OrdSubset,
	{
		fn mul_assign(&mut self, rhs: RHS) {
			self.0.mul_assign(rhs);
			if !cfg!(feature="unchecked_ops") {
				assert!(!self.0.is_outside_order(), "Result of {}= operation is outside order", "*");
			}
		}
	}

	impl<T, RHS> DivAssign<RHS> for OrdVar<T>
		where T: PartialOrd + PartialEq + DivAssign<RHS> + OrdSubset,
	{
		fn div_assign(&mut self, rhs: RHS) {
			self.0.div_assign(rhs);
			if !cfg!(feature="unchecked_ops") {
				assert!(!self.0.is_outside_order(), "Result of {}= operation is outside order", "/");
			}
		}
	}

	impl<T, RHS> RemAssign<RHS> for OrdVar<T>
		where T: PartialOrd + PartialEq + RemAssign<RHS> + OrdSubset,
	{
		fn rem_assign(&mut self, rhs: RHS) {
			self.0.rem_assign(rhs);
			if !cfg!(feature="unchecked_ops") {
				assert!(!self.0.is_outside_order(), "Result of {}= operation is outside order", "%");
			}
		}
	}

	impl<T, RHS> BitAndAssign<RHS> for OrdVar<T>
		where T: PartialOrd + PartialEq + BitAndAssign<RHS> + OrdSubset,
	{
		fn bitand_assign(&mut self, rhs: RHS) {
			self.0.bitand_assign(rhs);
			if !cfg!(feature="unchecked_ops") {
				assert!(!self.0.is_outside_order(), "Result of {}= operation is outside order", "&");
			}
		}
	}

	impl<T, RHS> BitOrAssign<RHS> for OrdVar<T>
		where T: PartialOrd + PartialEq + BitOrAssign<RHS> + OrdSubset,
	{
		fn bitor_assign(&mut self, rhs: RHS) {
			self.0.bitor_assign(rhs);
			if !cfg!(feature="unchecked_ops") {
				assert!(!self.0.is_outside_order(), "Result of {}= operation is outside order", "|");
			}
		}
	}

	impl<T, RHS> BitXorAssign<RHS> for OrdVar<T>
		where T: PartialOrd + PartialEq + BitXorAssign<RHS> + OrdSubset,
	{
		fn bitxor_assign(&mut self, rhs: RHS) {
			self.0.bitxor_assign(rhs);
			if !cfg!(feature="unchecked_ops") {
				assert!(!self.0.is_outside_order(), "Result of {}= operation is outside order", "^");
			}
		}
	}

	impl<T, RHS> ShlAssign<RHS> for OrdVar<T>
		where T: PartialOrd + PartialEq + ShlAssign<RHS> + OrdSubset,
	{
		fn shl_assign(&mut self, rhs: RHS) {
			self.0.shl_assign(rhs);
			if !cfg!(feature="unchecked_ops") {
				assert!(!self.0.is_outside_order(), "Result of {}= operation is outside order", "<<");
			}
		}
	}

	impl<T, RHS> ShrAssign<RHS> for OrdVar<T>
		where T: PartialOrd + PartialEq + ShrAssign<RHS> + OrdSubset,
	{
		fn shr_assign(&mut self, rhs: RHS) {
			self.0.shr_assign(rhs);
			if !cfg!(feature="unchecked_ops") {
				assert!(!self.0.is_outside_order(), "Result of {}= operation is outside order", ">>");
			}
		}
	}

	// ------------------------ unary ops ------------------------------------------
	impl<T> Neg for OrdVar<T>
		where T: PartialOrd + PartialEq + Neg,
			T::Output: PartialOrd + PartialEq + Debug + OrdSubset,
	{
		type Output = OrdVar<T::Output>;
		fn neg(self) -> Self::Output {
			construct(self.into_inner().neg())
		}
	}

	impl<T> Not for OrdVar<T>
		where T: PartialOrd + PartialEq + Not,
			T::Output: PartialOrd + PartialEq + Debug + OrdSubset,
	{
		type Output = OrdVar<T::Output>;
		fn not(self) -> Self::Output {
			construct(self.into_inner().not())
		}
	}
}