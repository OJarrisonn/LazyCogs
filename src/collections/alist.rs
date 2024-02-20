use std::{collections::LinkedList, mem};

use crate::{lazy::LazyClone, alc::Alc};

#[derive(Debug)]

pub struct AtomicLazyList<T: Clone> { 
    list: Alc<LinkedList<Alc<T>>>,
}

impl<T: Clone> AtomicLazyList<T> {
    /// Creates a new empty LazyList
    pub fn new() -> Self {
        Self{
            list: Alc::new(LinkedList::new())
        }
    }

    /// Obtains a reference to a specific value in the list
    /// 
    /// If the index is out of range it returns `None`
    /// 
    /// This operation is **always** O(n)
    pub fn get(&self, index: usize) -> Option<&T> {
        let list = self.list.read();
        
        list.iter().nth(index).map(Alc::read)
    }

    /// Obtains a mutable reference to a specific value in the list
    /// 
    /// If the index is out of range it returns `None`
    /// 
    /// This operation is protected, it means, that the other clones aren't affected
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let list = self.list.read_mut();
        
        list.iter_mut().nth(index).map(Alc::read_mut)
    }

    /// Obtains a lazy clone to a specific value in the list
    pub fn get_lazy(&self, index: usize) -> Option<Alc<T>> {
        self.list.iter().nth(index).cloned()
    }

    /// Changes the value at a given position of the list
    pub fn set(&mut self, index: usize, value: T) -> Result<(), ()>{
        if self.is_mutable() {
            // We can modify ourselves with no side effects
            
            let mut list = unsafe { 
                mem::replace(
                    &mut self.list,
                    Alc::new(LinkedList::new()))
                .destroy() 
            };

            let res = match list.iter_mut().nth(index) {
                Some(elem) => {
                    elem.write(value);
                    Ok(())
                },
                None => Err(()),
            };

            // Put the modified vector back in the structure
            self.list = Alc::new(list);

            res
        } else {
            // We need to clone the vector so we don't mess with other clones
            let mut list = self.list.take();

            let res = match list.iter_mut().nth(index) {
                Some(elem) => {
                    elem.write(value);
                    Ok(())
                },
                None => Err(()),
            };

            // We need to mutate ourselves, to use the new modified vector, 
            // and update our state to Mutable
            self.list = Alc::new(list);

            res
        }
    }

    /// Pushes a new element at the end of the list
    pub fn push_back(&mut self, value: T) {
        if self.is_mutable() {
            let mut list = unsafe {
                mem::replace(
                    &mut self.list, 
                    Alc::new(LinkedList::new()))
                    .destroy()
            };

            list.push_back(Alc::new(value));

            self.list = Alc::new(list);
        } else {
            let mut list = self.list.take();
            list.push_back(Alc::new(value));
            self.list = Alc::new(list);
        }
    }

    /// Pushes a new element at the beginning of the list
    pub fn push_front(&mut self, value: T) {
        if self.is_mutable() {
            let mut list = unsafe {
                mem::replace(
                    &mut self.list, 
                    Alc::new(LinkedList::new()))
                    .destroy()
            };

            list.push_front(Alc::new(value));

            self.list = Alc::new(list);
        } else {
            let mut list = self.list.take();
            list.push_front(Alc::new(value));
            self.list = Alc::new(list);
        }
    }

    #[inline(always)]
    /// Returns a reference to the first element of the list
    ///
    /// Return `None` if the list is empty
    pub fn front(&self) -> Option<&T> {
        self.list.read().front().map(Alc::read)
    }

    #[inline(always)]
    /// Returns a mutable reference to the first element of the list
    /// 
    /// Return `None` if the list is empty
    /// 
    /// This operation is protected, it means, that the other clones aren't affected
    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.list.read_mut().front_mut().map(Alc::read_mut)
    }

    #[inline(always)]
    /// Returns a lazy clone to the first element of the list
    ///
    /// Returns `None` if the list is empty
    pub fn front_lazy(&self) -> Option<Alc<T>> {
        self.list.read().front().cloned()
    }

    #[inline(always)]
    /// Returns a reference to the last element of the list
    ///
    /// Return `None` if the list is empty
    pub fn back(&self) -> Option<&T> {
        self.list.read().back().map(Alc::read)
    }

    #[inline(always)]
    /// Returns a mutable reference to the last element of the list
    ///
    /// Return `None` if the list is empty
    /// 
    /// This operation is protected, it means, that the other clones aren't affected
    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.list.read_mut().back_mut().map(Alc::read_mut)
    }

    #[inline(always)]
    /// Returns a lazy clone to the last element of the list
    ///
    /// Returns `None` if the list is empty
    pub fn back_lazy(&self) -> Option<Alc<T>> {
        self.list.read().back().cloned()
    }

    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let list = self.list.read();
        list.iter().map(Alc::read)
    }

    #[inline(always)]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        let list = self.list.read_mut();
        list.iter_mut().map(Alc::read_mut)
    }
}

impl<T: Clone> LazyClone for AtomicLazyList<T> {
    fn lazy(&self) -> Self {
        Self { 
            list: self.list.lazy()
        }
    }

    fn eager(&self) -> Self {
        Self { 
            list: self.list.eager()
        }
    }

    fn is_mutable(&self) -> bool {
        self.list.is_mutable()
    }
}

impl<T: Clone> From<LinkedList<Alc<T>>> for AtomicLazyList<T> {
    fn from(value: LinkedList<Alc<T>>) -> Self {
        Self { 
            list: Alc::new(value) 
        }
    }
}

impl<T: Clone> From<LinkedList<T>> for AtomicLazyList<T> {
    fn from(value: LinkedList<T>) -> Self {
        Self {
            list: Alc::new(value.into_iter()
                .map(Alc::new)
                .collect()
            ),
        }
    }
}

impl<T: Clone> From<Vec<Alc<T>>> for AtomicLazyList<T> {
    fn from(value: Vec<Alc<T>>) -> Self {
        Self { 
            list: Alc::new(value.into_iter().collect()) 
        }
    }
}

impl<T: Clone> From<Vec<T>> for AtomicLazyList<T> {
    fn from(value: Vec<T>) -> Self {
        Self {
            list: Alc::new(value.into_iter()
                .map(Alc::new)
                .collect()
            ),
        }
    }
}

impl<T: Clone> From<&[T]> for AtomicLazyList<T> {
    fn from(value: &[T]) -> Self {
        value.to_vec()
            .into_iter()
            .collect()
    }
}

impl<T: Clone> Into<LinkedList<Alc<T>>> for AtomicLazyList<T> {
    fn into(self) -> LinkedList<Alc<T>> {
        self.list.unwrap()
            .into_iter()
            .collect()
    }
}

impl<T: Clone> Into<LinkedList<T>> for AtomicLazyList<T> {
    fn into(self) -> LinkedList<T> {
        self.list.unwrap()
            .into_iter()
            .map(|elem| elem.unwrap())
            .collect()
    }
}

impl<T: Clone> FromIterator<T> for AtomicLazyList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        LinkedList::from_iter(iter).into()
    }
}

impl<T: Clone> IntoIterator for AtomicLazyList<T> {
    type Item = T;
    type IntoIter = std::collections::linked_list::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.list.take()
            .into_iter()
            .map(Alc::unwrap)
            .collect::<LinkedList<T>>().into_iter()
    }
}


#[cfg(test)]
mod tests {
    use std::{iter::zip, thread};

    use crate::lazy::LazyClone;

    use super::AtomicLazyList;

    #[test]
    fn create() {
        let mut lv = AtomicLazyList::<Box<str>>::from(&["Hello".into(), "World".into(), "Take".into(), "a look at this".into()] as &[Box<str>]);
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
        let mut lv = AtomicLazyList::from(vec!["HI", "Goodbye", "Farwell", "Hello"]);
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
        let lv = AtomicLazyList::from([String::from("rust"), String::from("mojo"), String::from("zig"), String::from("carbon"),String::from("aura")].to_vec());

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
        let lv: AtomicLazyList<_> = v.into_iter()
            .collect();

        dbg!(lv);
    }

    #[test]
    fn safety() {
        let l = AtomicLazyList::from(vec!["Hi".to_string(), "I love".to_string(), "grapes".to_string()]);
        let mut l_clone = l.lazy();
        

        let h2 = thread::spawn(move || 
            l_clone.iter_mut()
                .for_each(|elem| {
                    *elem = elem.to_ascii_uppercase();
                    println!("{}", elem);
                }));

        let h = thread::spawn(move|| 
            l.iter()
                .for_each(|elem| println!("{}", elem)));
                
        let _ = h2.join();
        let _ = h.join();
    }
}