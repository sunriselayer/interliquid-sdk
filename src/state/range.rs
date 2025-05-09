use std::ops::Bound;

// Object-safe version of RangeBounds
pub struct ObjectSafeRangeBounds<T> {
    start_bound: Bound<T>,
    end_bound: Bound<T>,
}

impl<T> ObjectSafeRangeBounds<T> {
    pub fn new(start_bound: Bound<T>, end_bound: Bound<T>) -> Self {
        Self {
            start_bound,
            end_bound,
        }
    }
}

impl<T> std::ops::RangeBounds<T> for ObjectSafeRangeBounds<T> {
    fn start_bound(&self) -> Bound<&T> {
        self.start_bound.as_ref()
    }

    fn end_bound(&self) -> Bound<&T> {
        self.end_bound.as_ref()
    }

    fn contains<U>(&self, _item: &U) -> bool
    where
        T: PartialOrd<U>,
        U: ?Sized + PartialOrd<T>,
    {
        unimplemented!()
    }
}
