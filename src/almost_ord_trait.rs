/// This trait is for types that have a total ordering except for some elements which fall outside the line.
/// ```is_outside_order()``` must return true for these outliers and false for anything else.
///
/// The method ```partial_cmp(a,b)``` must return ```Some(_)``` if ```is_outside_order(a)``` == ```is_outside_order(b)``` == ```false``` and ```None``` else.
pub trait AlmostOrd: PartialOrd<Self> + PartialEq<Self> {
	fn is_outside_order(&self) -> bool;

	#[inline(always)]
	fn is_on_order(&self) -> bool {
		!self.is_outside_order()
	}
}
/* Needs negative trait bounds
impl<T: Ord + !AlmostOrd> AlmostOrd for T {
	fn is_outside_order(&self) -> bool {
		false
	}
}
*/
impl<'a, A> AlmostOrd for &'a A where A: AlmostOrd {
	fn is_outside_order(&self) -> bool {
		(*self).is_outside_order()
	}
}

impl<'a, A> AlmostOrd for &'a mut A where A: AlmostOrd {
	fn is_outside_order(&self) -> bool {
		(**self).is_outside_order()
	}
}

impl AlmostOrd for f64 {
	fn is_outside_order(&self) -> bool {
		self.is_nan()
	}
}

impl AlmostOrd for f32 {
	fn is_outside_order(&self) -> bool {
		self.is_nan()
	}
}
