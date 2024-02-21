use std::{mem, ops::{Index, IndexMut}};

use crate::{lazy::LazyClone, lc::Lc};

#[derive(Debug)]
/// lazy-cogs implementation of a Vector. 
/// It's a collection meant to be used when you need to work with the individual elements
/// 
/// Cloning a LazyVec is always O(1). Getting elements from it is also O(1)
/// 
/// Modifing existing elements may take O(n) if the vector is a clone that is still modified, 
/// or if it has living clones.
/// 
/// Pushing elements follow the same logic
pub struct LazyVec<T: Clone> { 
    vec: Lc<Vec<Lc<T>>>,
}

impl<T: Clone> LazyVec<T> {
    /// Creates a new empty LazyVec
    pub fn new() -> Self {
        Self{
            vec: Lc::new(Vec::new())
        }
    }

    /// Obtains a reference to a specific value in the lazy vector
    /// 
    /// If the index is out of range it returns `None`
    /// 
    /// This operation is **always** O(1)
    pub fn get(&self, index: usize) -> Option<&T> {
        let vec = self.vec.read();
        
        vec.get(index).map(Lc::read)
    }

    /// Obtains a mutable reference to a specific value in the lazy vector
    /// 
    /// If the index is out of range it returns `None`
    /// 
    /// This operation is protected, it means, that the other clones aren't affected
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let vec = self.vec.read_mut();
        
        vec.get_mut(index).map(Lc::read_mut)
    }

    #[inline(always)]
    /// Obtains a lazy clone to a specific value in the lazy vector
    /// 
    /// If the index is out of range it returns `None`
    pub fn get_lazy(&self, index: usize) -> Option<Lc<T>> {
        self.vec.get(index).cloned()
    }

    /// Updates an item in the current vector
    /// 
    /// The operation coast dependents on the state of the vector:
    /// - If vector was never modified, this costs O(n)
    /// - If it was modified but some one cloned it, it's also O(n)
    /// - If it was modified and no one cloned it, it's O(1)
    /// - If it isn't cloned from other vector and no one cloned it, it's O(1)
    pub fn set(&mut self, index: usize, value: T) -> Result<(), ()>{
        let mut vec = if self.is_mutable() {
            // We can modify ourselves with no side effects
            unsafe { 
                mem::replace(
                    &mut self.vec,
                    Lc::new(Vec::new()))
                .destroy() 
            }

        } else {
            // We need to clone the vector so we don't mess with other clones
            self.vec.take()
        };

        let res = match vec.get_mut(index) {
            Some(elem) => {
                elem.write(value);
                Ok(())
            },
            None => Err(()),
        };
    
        // Put the modified vector back in the structure
        self.vec = Lc::new(vec);
    
        res
    }

    /// Pushes a new element at the end of the vector
    pub fn push(&mut self, value: T) {
        let mut vec = if self.is_mutable() {
            unsafe {
                mem::replace(
                    &mut self.vec, 
                    Lc::new(Vec::new()))
                    .destroy()
            }
        } else {
            self.vec.take()
        };
        
        vec.push(Lc::new(value));
        self.vec = Lc::new(vec);
    }

    /// Pops an element at the end of the vector
    pub fn pop(&mut self) -> Option<T> {
        let mut vec = if self.is_mutable() {
            unsafe {
                mem::replace(
                    &mut self.vec, 
                    Lc::new(Vec::new()))
                    .destroy()
            }
        } else {
            self.vec.take()
        };
        
        let res = vec.pop();
        self.vec = Lc::new(vec);
        res.map(Lc::unwrap)
    }

    #[inline(always)]
    /// Removes an element from the vector
    pub fn remove(&mut self, index: usize) -> T {
        self.remove_lazy(index).unwrap()
    }

    /// Removes an element from the vector and returns a lazy clone to it
    pub fn remove_lazy(&mut self, index: usize) -> Lc<T> {
        let mut vec = mem::replace(&mut self.vec, Lc::new(vec![])).unwrap();
        let res = vec.remove(index);

        self.vec = Lc::new(vec);

        res
    }

    /// Inserts an element at a given position in a vector
    pub fn insert(&mut self, index: usize, value: T) {
        let mut vec = mem::replace(&mut self.vec, Lc::new(vec![])).unwrap();
        vec.insert(index, value.into());

        self.vec = Lc::new(vec);
    }

    /// Produces an iterator over the elements
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let vec = self.vec.read();
        vec.iter().map(Lc::read)
    }

    /// Produces a mutable iterator
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        let vec = self.vec.read_mut();
        vec.iter_mut().map(Lc::read_mut)
    }
}

impl<T: Clone> LazyClone for LazyVec<T> {
    #[inline(always)]
    fn lazy(&self) -> Self {
        Self { 
            vec: self.vec.lazy()
        }
    }
    
    #[inline(always)]
    fn eager(&self) -> Self {
        Self { 
            vec: self.vec.eager()
        }
    }

    #[inline(always)]
    fn is_mutable(&self) -> bool {
        self.vec.is_mutable()
    }
}

impl<T: Clone> From<Vec<Lc<T>>> for LazyVec<T> {
    fn from(value: Vec<Lc<T>>) -> Self {
        Self { 
            vec: Lc::new(value) 
        }
    }
}

impl<T: Clone> From<Vec<T>> for LazyVec<T> {
    fn from(value: Vec<T>) -> Self {
        Self {
            vec: Lc::new(value.into_iter()
                .map(Lc::new)
                .collect()
            ),
        }
    }
}

impl<T: Clone> From<&[T]> for LazyVec<T> {
    fn from(value: &[T]) -> Self {
        value.to_vec().into()
    }
}

impl<T: Clone> Into<Vec<Lc<T>>> for LazyVec<T> {
    fn into(self) -> Vec<Lc<T>> {
        self.vec.unwrap()
            .into_iter()
            .collect()
    }
}

impl<T: Clone> Into<Vec<T>> for LazyVec<T> {
    fn into(self) -> Vec<T> {
        self.vec.unwrap()
            .into_iter()
            .map(|elem| elem.unwrap())
            .collect()
    }
}

impl<T: Clone> FromIterator<T> for LazyVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Vec::from_iter(iter).into()
    }
}

impl<T: Clone> IntoIterator for LazyVec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.take()
            .into_iter()
            .map(Lc::unwrap)
            .collect::<Vec<T>>().into_iter()
    }
}

impl<T: Clone> Index<usize> for LazyVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T: Clone> IndexMut<usize> for LazyVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use crate::lazy::LazyClone;

    use super::LazyVec;

    #[test]
    fn create() {
        let mut lv = LazyVec::<Box<str>>::from(&["Hello".into(), "World".into(), "Take".into(), "a look at this".into()] as &[Box<str>]);
        let mut lv2 = lv.lazy();
        let mut lv3 = lv2.lazy();

        let _ = lv.set(0, "Bye".into());
        let _ = lv2.set(2, "Give".into());
        let _ = lv3.set(1, "Värld".into());

        // Check if the boxes are the same, so we are safely reusing memory
        assert_eq!(lv.get(1).unwrap().as_ref() as *const str, lv2.get(1).unwrap().as_ref() as *const str);
        // Check if the boxes are not the same, so we cloned what was needed
        assert_ne!(lv.get(1).unwrap().as_ref() as *const str, lv3.get(1).unwrap().as_ref() as *const str);
    }

    #[test]
    #[allow(unused_mut)]
    #[allow(unused_results)]
    #[allow(unused_must_use)]
    fn mutability_check() {
        let mut lv = LazyVec::from(vec!["HI", "Goodbye", "Farwell", "Hello"]);
        let mut lv2 = lv.lazy();
        let mut lv3 = lv2.lazy();
        let mut lv4 = lv2.lazy();
        
        lv2.set(3, "Halo");
        lv.set(0, "Hej");

        assert!(lv.is_mutable());
        assert!(lv2.is_mutable());
        assert!(!lv3.is_mutable());
        assert!(!lv4.is_mutable());

        for e in zip(zip(lv.iter(), lv2.iter()), zip(lv3.iter(), lv4.iter())) {
            println!("{:?} : {:?} : {:?} : {:?}", e.0.0.as_ptr(), e.0.1.as_ptr(), e.1.0.as_ptr(), e.1.1.as_ptr())
        }
    }

    #[test]
    fn iterators() {
        let lv = LazyVec::from([String::from("rust"), String::from("mojo"), String::from("zig"), String::from("carbon"),String::from("aura")].to_vec());

        lv.iter()
            .map(|elem| elem.to_uppercase())
            .for_each(|elem| println!("{:?}", elem));

        lv.into_iter()
            .map(|elem| elem
                .chars()
                .rev()
                .collect::<String>()
            )
            .for_each(|elem| println!("{:?}", elem));
    }

    #[test]
    fn collecting() {
        let v = vec!["Hi", "my", "name", "is", "something"];
        let lv: LazyVec<_> = v.into_iter()
            .collect();

        dbg!(lv);
    }
}