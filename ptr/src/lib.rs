use std::{fmt::Display, ops::{Deref, DerefMut}};

pub struct MyBox<T> where T: Display {
    data: T
}

impl<T> MyBox<T> where T: Display {
    pub fn new(data: T) -> MyBox<T>  {
        MyBox {
            data
        }
    }
}

impl<T> Deref for MyBox<T> where T: Display {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for MyBox<T> where T: Display {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> Drop for MyBox<T> where T: Display {
    fn drop(&mut self) {
        println!("It has been dropped: {}", self.data);
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::{Rc, Weak}};

    use super::*;

    #[test]
    fn box_test() {
        let mut b = MyBox::new(123);
        *b = 22;
        assert_eq!(*b, 22);
    }

    #[test]
    fn multiple_owners() {
        let hello = Rc::<String>::new(String::from("Hello world"));
        let another_hello = Rc::clone(&hello);
        let _yet_another_hello = Rc::clone(&hello);

        assert_eq!(Rc::strong_count(&hello), 3);

        { another_hello };

        assert_eq!(Rc::strong_count(&hello), 2);
    }

    #[test]
    fn multiple_mutable_owners() {
        let value = Rc::new(RefCell::new(String::new()));
        let another_ref = value.clone();

        let mut ref_mut = value.borrow_mut();
        ref_mut.push('a');
        { ref_mut }; // Return the mutable borrow

        assert_eq!(*another_ref.borrow(), "a");
    }

    #[test]
    fn ouroboros() {
        struct Cons {
            _data: i32,
            next: Option<Rc<RefCell<Cons>>>
        }
        let first = Rc::new(RefCell::new(Cons {
            _data: 0,
            next: None
        }));
        let second = Rc::new(RefCell::new(Cons {
            _data: 1,
            next: Some(first.clone())
        }));
        let mut first_mut = first.borrow_mut();
        first_mut.next = Some(second.clone());

        assert_eq!(Rc::strong_count(&first), 2);
        assert_eq!(Rc::strong_count(&second), 2);

        // This results in an ouroboros, where they both have references to each other,
        // and is thus not freed... forever.
    }

    #[test]
    fn tree() {
        #[derive(Clone)]
        struct Node {
            data: i32,
            children: RefCell<Vec<Rc<Node>>>,
            father: RefCell<Weak<Node>>
        }

        let leaf = Rc::new(Node {
            data: 0,
            children: RefCell::new(vec![]),
            father: RefCell::new(Weak::new())
        });

        let root = Rc::new(Node {
            data: 1,
            children: RefCell::new(vec![leaf.clone()]),
            father: RefCell::new(Weak::new())
        });

        let mut leaf_father_mut = leaf.father.borrow_mut();
        *leaf_father_mut = Rc::downgrade(&root);

        assert_eq!(Rc::strong_count(&root), 1);
        assert_eq!(Rc::strong_count(&leaf), 2);
        assert_eq!(Rc::weak_count(&root), 1);
    }
}
