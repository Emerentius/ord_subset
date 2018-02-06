// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
// http://opensource.org/licenses/MIT, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Trait for types that form a total order when a few values are disallowed.
///
/// `is_outside_order()` must return `true` for these outliers and `false` for anything else.
///
/// `std::cmp::PartialOrd::partial_cmp(a,b)` must return `Some(_)` if a,b are both inside order and `None` if only one is outside order. Return value for two variables outside order is undefined.
pub trait OrdSubset: PartialOrd<Self> + PartialEq<Self> {
    fn is_outside_order(&self) -> bool;
}

impl<'a, A> OrdSubset for &'a A
where
    A: OrdSubset,
{
    #[inline(always)]
    fn is_outside_order(&self) -> bool {
        (**self).is_outside_order()
    }
}

impl<'a, A> OrdSubset for &'a mut A
where
    A: OrdSubset,
{
    #[inline(always)]
    fn is_outside_order(&self) -> bool {
        (**self).is_outside_order()
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(float_cmp, eq_op))]
impl OrdSubset for f64 {
    #[inline(always)]
    fn is_outside_order(&self) -> bool {
        // only NaNs != itself
        *self != *self
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(float_cmp, eq_op))]
impl OrdSubset for f32 {
    #[inline(always)]
    fn is_outside_order(&self) -> bool {
        // only NaNs != itself
        *self != *self
    }
}

trait EnsureOrd: Ord {}

macro_rules! impl_for_ord {
	($($type:ty),+) => (
		$(
			// safe guard against incorrect macro invocation
			// `where Self: Ord` on OrdSubset impl would be rendered in docs
			impl EnsureOrd for $type {}

			impl OrdSubset for $type
			{
				#[inline(always)]
				fn is_outside_order(&self) -> bool {
					false
				}
			}
		)+
	)
}

#[cfg_attr(rustfmt, rustfmt_skip)]
impl_for_ord!((), u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, bool, char,
    ::core::fmt::Error, ::core::cmp::Ordering, ::core::any::TypeId);

#[cfg(feature = "std")]
impl_for_ord!(
    String, ::std::ffi::CString, ::std::ffi::CStr, ::std::ffi::OsString, ::std::ffi::OsStr,
    ::std::time::SystemTime, ::std::time::Instant, ::std::time::Duration, ::std::path::Path,
    ::std::path::PathBuf, ::std::net::Ipv6Addr, ::std::net::Ipv4Addr, ::std::net::IpAddr,
    ::std::io::ErrorKind
);

impl<T: ?Sized> OrdSubset for ::core::marker::PhantomData<T> {
    #[inline(always)]
    fn is_outside_order(&self) -> bool {
        false
    }
}

impl<T: ?Sized> OrdSubset for *mut T {
    #[inline(always)]
    fn is_outside_order(&self) -> bool {
        false
    }
}

impl<T: ?Sized> OrdSubset for *const T {
    #[inline(always)]
    fn is_outside_order(&self) -> bool {
        false
    }
}

impl<T: OrdSubset> OrdSubset for ::core::cmp::Reverse<T> {
    #[inline(always)]
    fn is_outside_order(&self) -> bool {
        self.0.is_outside_order()
    }
}

impl<T: OrdSubset> OrdSubset for Option<T> {
    #[inline(always)]
    fn is_outside_order(&self) -> bool {
        self.as_ref().map_or(false, OrdSubset::is_outside_order)
    }
}

impl<T: OrdSubset + Copy> OrdSubset for ::core::cell::Cell<T> {
    #[inline(always)]
    fn is_outside_order(&self) -> bool {
        self.get().is_outside_order()
    }
}

impl<T: OrdSubset + ?Sized> OrdSubset for ::core::cell::RefCell<T> {
    #[inline(always)]
    fn is_outside_order(&self) -> bool {
        self.borrow().is_outside_order()
    }
}

impl<T: OrdSubset> OrdSubset for ::core::mem::ManuallyDrop<T> {
    #[inline(always)]
    fn is_outside_order(&self) -> bool {
        (**self).is_outside_order()
    }
}

impl<T: OrdSubset> OrdSubset for ::core::num::Wrapping<T> {
    #[inline(always)]
    fn is_outside_order(&self) -> bool {
        self.0.is_outside_order()
    }
}

#[cfg(feature = "std")]
impl<T: OrdSubset> OrdSubset for Vec<T> {
    #[inline(always)]
    fn is_outside_order(&self) -> bool {
        self.iter().any(OrdSubset::is_outside_order)
    }
}

#[cfg(feature = "std")]
impl<T: OrdSubset + ?Sized> OrdSubset for Box<T> {
    #[inline(always)]
    fn is_outside_order(&self) -> bool {
        (**self).is_outside_order()
    }
}

#[cfg(feature = "std")]
impl<T: OrdSubset + ?Sized> OrdSubset for ::std::sync::Arc<T> {
    #[inline(always)]
    fn is_outside_order(&self) -> bool {
        (**self).is_outside_order()
    }
}

#[cfg(feature = "std")]
impl<T: OrdSubset + ?Sized> OrdSubset for ::std::rc::Rc<T> {
    #[inline(always)]
    fn is_outside_order(&self) -> bool {
        (**self).is_outside_order()
    }
}

#[cfg(feature = "std")]
impl<'a, T: OrdSubset + ?Sized + ToOwned> OrdSubset for ::std::borrow::Cow<'a, T> {
    #[inline(always)]
    fn is_outside_order(&self) -> bool {
        (**self).is_outside_order()
    }
}

#[cfg(feature = "std")]
impl<T: OrdSubset> OrdSubset for ::std::collections::BTreeSet<T> {
    #[inline(always)]
    fn is_outside_order(&self) -> bool {
        self.iter().any(OrdSubset::is_outside_order)
    }
}

#[cfg(feature = "std")]
impl<K: OrdSubset, V: OrdSubset> OrdSubset for ::std::collections::BTreeMap<K, V> {
    #[inline(always)]
    fn is_outside_order(&self) -> bool {
        self.iter().any(|(k, v)| k.is_outside_order() || v.is_outside_order())
    }
}

#[cfg(feature = "std")]
impl<T: OrdSubset> OrdSubset for ::std::collections::VecDeque<T> {
    #[inline(always)]
    fn is_outside_order(&self) -> bool {
        self.iter().any(OrdSubset::is_outside_order)
    }
}

#[cfg(feature = "std")]
impl<T: OrdSubset> OrdSubset for ::std::collections::LinkedList<T> {
    #[inline(always)]
    fn is_outside_order(&self) -> bool {
        self.iter().any(OrdSubset::is_outside_order)
    }
}

macro_rules! array_impls {
    ($($N:expr),+) => {
        $(
			impl<T: OrdSubset> OrdSubset for [T; $N] {
				#[inline(always)]
				fn is_outside_order(&self) -> bool {
					(&self[..]).is_outside_order()
				}
			}
        )+
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
array_impls!(
	0, 1, 2, 3, 4, 5, 6, 7, 8,
	9, 10, 11, 12, 13, 14, 15, 16,
	17, 18, 19, 20, 21, 22, 23, 24,
	25, 26, 27, 28, 29, 30, 31, 32
);

impl<T: OrdSubset> OrdSubset for [T] {
    #[inline(always)]
    fn is_outside_order(&self) -> bool {
        self.iter().any(OrdSubset::is_outside_order)
    }
}

// code stolen from std library
macro_rules! tuple_impls {
    ($(
        $Tuple:ident {
            $(($idx:tt) -> $T:ident)+
        }
    )+) => {
        $(
            impl<$($T:OrdSubset),+> OrdSubset for ($($T,)+) where last_type!($($T,)+): ?Sized {
                #[inline]
                fn is_outside_order(&self) -> bool {
                    $(self.$idx.is_outside_order())||+
                }
            }
        )+
    }
}

macro_rules! last_type {
    ($a:ident,) => { $a };
    ($a:ident, $($rest_a:ident,)+) => { last_type!($($rest_a,)+) };
}

tuple_impls! {
    Tuple1 {
        (0) -> A
    }
    Tuple2 {
        (0) -> A
        (1) -> B
    }
    Tuple3 {
        (0) -> A
        (1) -> B
        (2) -> C
    }
    Tuple4 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
    }
    Tuple5 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
    }
    Tuple6 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
    }
    Tuple7 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
    }
    Tuple8 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
    }
    Tuple9 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
    }
    Tuple10 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
    }
    Tuple11 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
        (10) -> K
    }
    Tuple12 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
        (10) -> K
        (11) -> L
    }
}

// Small helper used a lot in sorts
pub(crate) trait CmpUnwrap: OrdSubset {
    #[inline(always)]
    fn cmp_unwrap(&self, other: &Self) -> ::core::cmp::Ordering {
        self.partial_cmp(other).expect(
            "Violated OrdSubset contract: a.partial_cmp(b) == None for a,b inside total order",
        )
    }
}

impl<T: OrdSubset> CmpUnwrap for T {}

// The tests here are primarily compile time tests
// If the tuple macros were wrong, it would show up in the std library
#[cfg(test)]
mod test {
    use super::OrdSubset;
    #[test]
    fn heterogenous_tuple() {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let tup = ((), 0u8, 0u16, 0u32, 0u64, 0usize, 0i8, 0i16, 0i32, 0i64, 0isize, 'a');
        assert!(!tup.is_outside_order());
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn slice() {
        let a = [0u8; 32];
        assert!( ! a.is_outside_order() );
        assert!( ! a.as_ref().is_outside_order() );
    }
}
