use std::cmp::Ordering::{self, Greater, Less};

#[derive(Debug, PartialEq, Eq)]
// None > Some, always
pub struct RevOption<T>(pub Option<T>);

impl<T: PartialOrd> PartialOrd<RevOption<T>> for RevOption<T> {
	//#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match (self.0.is_none(), other.0.is_none()) {
			(true, false) => Some(Greater),
			(false, true) => Some(Less),
			_             => self.0.partial_cmp(&other.0),
		}
	}
}

impl<T: Ord> Ord for RevOption<T> {
	//#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering {
		match (self.0.is_none(), other.0.is_none()) {
			(true, false) => Greater,
			(false, true) => Less,
			_             => self.0.cmp(&other.0),
		}
	}
}

#[test]
fn rev_option() {
	assert!( RevOption(None)   >  RevOption(Some(2)));
	assert!( RevOption(None)   >= RevOption(Some(2)));
	assert!( !(RevOption(None) <  RevOption(Some(2))));
}

#[test]
fn normal_option() {
	assert!( None   <  Some(2));
	assert!( None   <= Some(2));
	assert!(!( None >  Some(2) ) );
}
