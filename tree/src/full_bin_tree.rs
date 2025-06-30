use std::{cell::{Ref, RefCell}, rc::{Rc, Weak}};


pub struct Node<T>
where T: Clone + Eq + PartialOrd {
    parent : Option<Weak<RefCell<Node<T>>>>,
    left : Option<Rc<RefCell<Node<T>>>>,
    right : Option<Rc<RefCell<Node<T>>>>,
    value : T
}

impl<T> Node<T> 
where T: Clone + Eq + PartialOrd {

    fn contains(&self, search : &T) -> bool {
        if self.value == *search {return true;}
        else if search < &self.value {
            if let Some(left) = &self.left {
                return left.borrow().contains(search);
            }
        } else {
            if let Some(right) = &self.right {
                return right.borrow().contains(search);
            }
        }
        false
    }

    fn insert_at(&mut self, parent_link : Weak<RefCell<Node<T>>>,search : T) {
        let new_node = Rc::new(RefCell::new(Node {
                parent : Some(parent_link),
                value : search,
                left : None,
                right : None
        }));
        if self.value < new_node.borrow().value {
            self.right = Some(new_node);
        } else {
            self.left = Some(new_node);
        }
    }

    fn find_insert(&mut self, parent_link : Weak<RefCell<Node<T>>>, search : T) {
        if self.value == search { return; }
        if self.value > search {
            match self.left.as_mut() {
                None => self.insert_at(parent_link, search),
                Some(left) => {
                    let new_parent_link = Rc::downgrade(left);
                    return left.borrow_mut().find_insert(new_parent_link, search);
                }
            }
        } else {
            match self.right.as_mut() {
                None => self.insert_at(parent_link, search),
                Some(right) => {
                    let new_parent_link = Rc::downgrade(right);
                    return right.borrow_mut().find_insert(new_parent_link, search);
                }
            }
        }
    }
}

pub struct FullBinTree<T>
where T: Clone + Eq + PartialOrd {
    root : Option<Rc<RefCell<Node<T>>>>
}

impl<T> FullBinTree<T> 
where T: Clone + Eq + PartialOrd {
    pub fn new() -> Self {
        FullBinTree { root : None}
    }

    // pub fn min(&self) -> Option<&T> {
    //     let mut last: &Option<Rc<RefCell<Node<T>>>> = &None;
    //     let mut current = &self.root;
    //     while current.is_some() {
    //         last = current;
    //         current = &current.as_ref().unwrap().borrow().left;
    //     }
    //     match last {
    //         Some(x) => Some(&x.borrow().value),
    //         None => None
    //     }
    // }

    // pub fn max(&self) -> Option<&T> {
    //     let mut last: &Option<Rc<RefCell<Node<T>>>> = &None;
    //     let mut current = &self.root;
    //     while let Some(node) = current {
    //         last = current;
    //         current = &node.borrow().right;
    //     }
    //     return last.as_ref();
    // }

    pub fn insert(&mut self, value : T) {
        match &mut self.root {
            None => {
                self.root = Some(Rc::new(RefCell::new(Node {
                    parent : None,
                    value : value,
                    left : None,
                    right : None
                })));
            },
            Some(root) => {
                let parent_link = Rc::downgrade(root);
                root.borrow_mut().find_insert(parent_link, value); 
            }
        };
        
    }

    pub fn contains(&self, elem : &T) -> bool {
        match &self.root {
            None => false,
            Some(node) => node.borrow().contains(elem)
        }
    }

    pub fn delete(&mut self, value : T) {
        panic!("Not Implemented");
    }
}   




#[cfg(test)]
mod tests {
    use super::*;


    // #[test]
    // fn min_max () {
    //     let mut tree = FullBinTree::<u64>::new();
        
    //     assert_eq!(tree.min(), None);
    //     assert_eq!(tree.max(), None);
        
    //     tree.insert(5);

    //     assert_eq!(tree.min(), Some(5));
    //     assert_eq!(tree.max(), Some(5));

    //     tree.insert(2);
    //     tree.insert(9);

    //     assert_eq!(tree.min(), Some(2));
    //     assert_eq!(tree.max(), Some(9));
    // }

    #[test]
    fn contain() {
        let mut tree = FullBinTree::<u64>::new();
        
        tree.insert(5);
        tree.insert(2);
        tree.insert(9);


        assert!(tree.contains(&9), "tree should contain 9");
        assert!(tree.contains(&2), "tree should contain 2");
        assert!(tree.contains(&5), "tree should contain 5");
        assert!(!tree.contains(&4), "tree should not contain 4");
    }

    #[test]
    fn delete() {
        
        let mut tree = FullBinTree::<u64>::new();
        
        tree.insert(5);
        tree.insert(2);
        tree.insert(9);

        assert!(tree.contains(&5), "tree should contain 5");

        tree.delete(5);

        assert!(!tree.contains(&5));
        assert!(tree.contains(&9));
    }
}