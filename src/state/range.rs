use std::ops::{Bound, Range, RangeInclusive};

// Object-safe version of RangeBounds
pub struct RangeBounds<'a, T: ?Sized> {
    start_bound: Bound<&'a T>,
    end_bound: Bound<&'a T>,
}

impl<'a, T> std::ops::RangeBounds<T> for RangeBounds<'a, T> {
    fn start_bound(&self) -> Bound<&T> {
        self.start_bound
    }

    fn end_bound(&self) -> Bound<&T> {
        self.end_bound
    }

    fn contains<U>(&self, _item: &U) -> bool
    where
        T: PartialOrd<U>,
        U: ?Sized + PartialOrd<T>,
    {
        unimplemented!()
    }
}

impl<'a, T> From<&'a Range<T>> for RangeBounds<'a, T> {
    fn from(range: &'a Range<T>) -> Self {
        RangeBounds {
            start_bound: Bound::Included(&range.start),
            end_bound: Bound::Excluded(&range.end),
        }
    }
}

impl<'a, T> From<&'a RangeInclusive<T>> for RangeBounds<'a, T> {
    fn from(range: &'a RangeInclusive<T>) -> Self {
        RangeBounds {
            start_bound: Bound::Included(range.start()),
            end_bound: Bound::Included(range.end()),
        }
    }
}
