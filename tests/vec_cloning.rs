#[cfg(test)]
mod tests {
    use std::{iter::zip, rc::Rc};

    #[test]
    fn clone_vector() {
        let v1 = Rc::new(vec![1, 2, 3, 4].into_iter().map(Box::new).map(Rc::new).collect::<Vec<Rc<Box<i32>>>>());
        let v2 = Rc::as_ref(&v1).clone();

        for (e1, e2) in zip(v1.iter(), v2.iter()) {
            println!("{:?}, {:?}", e1.as_ref().as_ref() as *const i32, e2.as_ref().as_ref() as *const i32);
        }
    }
}