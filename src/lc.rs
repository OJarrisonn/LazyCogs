use std::{borrow::{Borrow, BorrowMut}, ops::{Deref, DerefMut}, rc::Rc};

use crate::lazy::LazyClone;

#[derive(Debug)]
/// Lc is a LazyClone wrapper, to provide lazy cloning for any data that doesn't implement `LazyClone` trait
/// 
/// If you're able to implement LazyClone for the data that you need, do it, if not, use this wrapper
pub struct Lc<T> (Rc<T>);

impl<T: Clone> Lc<T> {
    #[inline(always)]
    /// Creates a new Lc from a value
    pub fn new(value: T) -> Self {
        Self(Rc::new(value))
    }

    #[inline(always)]
    /// Returns a reference to the lazy cloned value
    pub fn read(&self) -> &T {
        &self.0
    }

    #[inline(always)]
    /// Ensures that the lazily cloned value is mutable and returns a mutable reference to it
    pub fn read_mut(&mut self) -> &mut T {
        if !self.is_mutable() {
            *self = self.eager();
        }

        assert!(self.is_mutable());

        Rc::get_mut(&mut self.0).unwrap()
    }

    #[inline(always)]
    /// Replaces the cloned value by another
    /// 
    /// Does not affect any value lazily cloned from this value
    pub fn write(&mut self, value: T) {
        self.0 = Rc::new(value);
    }

    #[inline(always)]
    /// Does an actual clone of the contained value
    /// 
    /// This clone may be expensive
    pub fn take(&self) -> T {
        self.0.as_ref().clone()
    }

    #[inline(always)]
    /// Checks if two Lc are pointing to the same data
    pub fn ptr_eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }

    /// Unwraps the lazy clone and returns the inner data in O(1). 
    /// But it only works if the Lc hasn't been lazily cloned, otherwise it panics
    /// 
    /// # Panics
    /// 
    /// If the struct is lazily clonning other structures or if some one is lazily cloning the structure
    pub unsafe fn destroy(self) -> T {
        if !self.is_mutable() {
            panic!("Destroyed a lazy clone that was being shared, this is invalid.")
        }
        Rc::into_inner(self.0).expect("Destroyed a lazy clone that was being shared, this is invalid.")
    }

    /// Unwraps the lazy clone and returns the inner data in O(1) if the Lc is mutable, otherwise performes an clone.
    pub fn unwrap(self) -> T {
        if self.is_mutable() { 
            unsafe { self.destroy() } 
        } else  {
            self.take()
        }
    }
}

impl<T: Clone> Deref for Lc<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.read()
    }
}

impl<T: Clone> DerefMut for Lc<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.read_mut()
    }
}

impl<T: Clone> LazyClone for Lc<T> {
    #[inline(always)]
    fn lazy(&self) -> Self {
        Self(Rc::clone(&self.0))
    }

    #[inline(always)]
    fn eager(&self) -> Self {
        Self(Rc::new(self.take()))
    }

    #[inline(always)]
    fn is_mutable(&self) -> bool {
        Rc::strong_count(&self.0) == 1
    }    
}

impl<T: Clone> Clone for Lc<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        LazyClone::lazy(&self)
    }
}

impl<T: Clone> From<T> for Lc<T> {
    #[inline(always)]
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: Clone> Borrow<T> for Lc<T> {
    #[inline(always)]
    fn borrow(&self) -> &T {
        self.0.borrow()
    }
}

impl<T: Clone> BorrowMut<T> for Lc<T> {
    #[inline(always)]
    fn borrow_mut(&mut self) -> &mut T {
        self.read_mut()
    }
}