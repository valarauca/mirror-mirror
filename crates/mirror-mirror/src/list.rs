use alloc::boxed::Box;
use core::fmt;

use crate::array::Array;
use crate::Reflect;
use crate::FromReflectError;

/// A reflected list type.
pub trait List: Array {
    /// Appends an element to the back of a collection.
    ///
    /// Returns `Err(_)` if `element` couldn't be parsed to the element type, using
    /// `FromReflect::from_reflect`.
    fn try_push(&mut self, element: &dyn Reflect) -> Result<(), FromReflectError>;

    /// Removes the last element from a vector and returns it, or `None` if it is empty.
    fn pop(&mut self) -> Option<Box<dyn Reflect>>;

    /// Removes and returns the element at position `index` within the vector, shifting all elements
    /// after it to the left.
    ///
    /// Returns `None` if `inde` is out of bounds.
    fn try_remove(&mut self, index: usize) -> Option<Box<dyn Reflect>>;

    /// Inserts an element at position `index` within the vector, shifting all elements after it to
    /// the right.
    ///
    /// Returns `Err(_)` if `element` couldn't be parsed to the element type, using
    /// `FromReflect::from_reflect`.
    fn try_insert(&mut self, index: usize, element: &dyn Reflect) -> Result<(), FromReflectError>;
}

impl fmt::Debug for dyn List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_reflect().debug(f)
    }
}
