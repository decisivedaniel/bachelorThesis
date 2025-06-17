use std::rc::{Rc, Weak};


pub struct Node<T>
where T: Copy + Eq + PartialOrd {
    parent : Option<Weak<Node<T>>>,
    left : Option<Rc<Node<T>>>,
    right : Option<Rc<Node<T>>>,
    value : T
}

pub struct FullBinTree<T>
where T: Copy + Eq + PartialOrd {
    root : Option<Rc<Node<T>>>
}

impl<T> FullBinTree<T> 
where T: Copy + Eq + PartialOrd {
    pub fn new() -> Self {
        FullBinTree { root : None}
    }

    pub fn min(&self) -> Option<T> {
        let mut last: &Option<Rc<Node<T>>> = &None;
        let mut current = &self.root;
        while current.is_some() {
            last = current;
            current = &current.as_ref().unwrap().left;
        }
        match last {
            Some(x) => Some(x.value),
            None => None
        }
    }

    pub fn max(&self) -> Option<T> {
        let mut last: &Option<Rc<Node<T>>> = &None;
        let mut current = &self.root;
        while current.is_some() {
            last = current;
            if current.as_ref().unwrap().right.is_some() {
                current = &current.as_ref().unwrap().right;
            }
        }
        match last {
            Some(x) => Some(x.value),
            None => None
        }
    }

    pub fn insert(&mut self, value : T) {
        let mut current = &mut self.root;
        while current.is_some() {
            if current.as_ref().unwrap().value == value {return;}
            else if current.as_ref().unwrap().value > value && current.as_ref().unwrap().left.is_some() {
                current = &mut current.as_mut().unwrap().left;
            }
            else if current.as_ref().unwrap().value < value && current.as_ref().unwrap().right.is_some() {
                current = &mut current.as_mut().unwrap().right;
            }
            else {
                break;
            }
        }
        let new_node = Some(Rc::new(Node {
                parent : None,
                value : value,
                left : None,
                right : None
        }));
        if current.is_none() {
            self.root = new_node
        }
        else if current.as_ref().unwrap().value < value {
            let left = current.as_mut().unwrap().left.take();
            current.replace(Rc::new(Node {
                parent : Some(Rc::downgrade(current.as_ref().unwrap())),
                value : current.as_ref().unwrap().value,
                left : left,
                right : new_node
            }));
        }
        else {
            let right = current.as_ref().unwrap().right  .take();
            current.replace(Rc::new(Node {
                parent : Some(Rc::downgrade(current.as_ref().unwrap())),
                value : current.as_ref().unwrap().value,
                left : new_node,
                right : right
            }));
        }
    }

    pub fn contains(&self, elem : T) -> bool {
        let mut current = &self.root;
        while current.is_some() {
            if current.as_ref().unwrap().value == elem {
                return true;
            }
            if current.as_ref().unwrap().value < elem {
                current = &current.as_ref().unwrap().right;
            }
            else {
                current = &current.as_ref().unwrap().left;
            }
        }
        false
    }

    pub fn delete(&mut self, value : T) {
        panic!("Not Implemented");
    }
}   




#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn min_max () {
        let mut tree = FullBinTree::<u64>::new();
        
        assert_eq!(tree.min(), None);
        assert_eq!(tree.max(), None);
        
        tree.insert(5);

        assert_eq!(tree.min(), Some(5));
        assert_eq!(tree.max(), Some(5));

        tree.insert(2);
        tree.insert(9);

        assert_eq!(tree.max(), Some(9));
        assert_eq!(tree.min(), Some(2));
    }

    #[test]
    fn contain() {
        let mut tree = FullBinTree::<u64>::new();
        
        tree.insert(5);
        tree.insert(2);
        tree.insert(9);


        assert!(tree.contains(9), "tree should contain 9");
        assert!(tree.contains(2), "tree should contain 2");
        assert!(tree.contains(5), "tree should contain 5");
        assert!(!tree.contains(4), "tree should not contain 4");
    }

    #[test]
    fn delete() {
        
        let mut tree = FullBinTree::<u64>::new();
        
        tree.insert(5);
        tree.insert(2);
        tree.insert(9);

        assert!(tree.contains(5), "tree should contain 5");

        tree.delete(5);

        assert!(!tree.contains(5));
        assert!(tree.contains(9));
    }
}