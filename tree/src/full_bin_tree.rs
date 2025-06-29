// use std::{cell::{Ref, RefCell}, rc::{Rc, Weak}};


// pub struct Node<T>
// where T: Clone + Eq + PartialOrd {
//     parent : Option<Weak<RefCell<Node<T>>>>,
//     left : Option<Rc<RefCell<Node<T>>>>,
//     right : Option<Rc<RefCell<Node<T>>>>,
//     value : T
// }

// pub struct FullBinTree<T>
// where T: Clone + Eq + PartialOrd {
//     root : Option<Rc<RefCell<Node<T>>>>
// }

// impl<T> FullBinTree<T> 
// where T: Clone + Eq + PartialOrd {
//     pub fn new() -> Self {
//         FullBinTree { root : None}
//     }

//     pub fn min(&self) -> Option<&T> {
//         let mut last: &Option<Rc<RefCell<Node<T>>>> = &None;
//         let mut current = &self.root;
//         while current.is_some() {
//             last = current;
//             current = &current.as_ref().unwrap().borrow().left;
//         }
//         match last {
//             Some(x) => Some(&x.borrow().value),
//             None => None
//         }
//     }

//     pub fn max(&self) -> Option<&T> {
//         let mut last: &Option<Rc<RefCell<Node<T>>>> = &None;
//         let mut current = &self.root;
//         while let Some(node) = current {
//             last = current;
//             current = &node.borrow().right;
//         }
//         return last.as_ref();
//     }

//     fn findInsert(&self, node : &Rc<RefCell<Node<'a, T>>>, search : &T) -> Option<&Rc<RefCell<Node<'a, T>>>> {
//         if node.borrow().value == *search { return None; }
//         if node.borrow().value > *search {
//             match &node.borrow().left {
//                 None => return Some(node),
//                 Some(left) => {
//                     return self.findInsert(left, search)
//                 }
//             }
//         } else {
//             match &node.borrow().right {
//                 None => return Some(node),
//                 Some(right) => {
//                     return self.findInsert(right, search)
//                 }
//             }
//         }
//     }

//     pub fn insert(&mut self, value : T) {
//         let current = match &self.root {
//             None => None,
//             Some(root) => self.findInsert(root, &value) 
//         };
//         let new_node = Rc::new(RefCell::new(Node {
//                 parent : None,
//                 value : value,
//                 left : None,
//                 right : None
//         }));
//         match current {
//             None => self.root = Some(new_node),
//             Some(parent_node) => {
//                 if parent_node.borrow().value < new_node.borrow().value {
//                     new_node.borrow_mut().parent = Some(Rc::downgrade(&parent_node));
//                     parent_node.borrow_mut().right = Some(new_node);
//                 } else {
//                     new_node.borrow_mut().parent = Some(Rc::downgrade(&parent_node));
//                     parent_node.borrow_mut().left = Some(new_node);
//                 }
//             }
//         }
//     }

//     pub fn contains(&self, elem : T) -> bool {
//         let mut current = &self.root;
//         while let Some(node) = current {
//             if node.borrow().value == elem {
//                 return true;
//             }
//             if node.borrow().value < elem {
//                 current = &node.borrow().right;
//             }
//             else {
//                 current = &node.borrow().left;
//             }
//         }
//         false
//     }

//     pub fn delete(&mut self, value : T) {
//         panic!("Not Implemented");
//     }
// }   




// #[cfg(test)]
// mod tests {
//     use super::*;


//     #[test]
//     fn min_max () {
//         let mut tree = FullBinTree::<u64>::new();
        
//         assert_eq!(tree.min(), None);
//         assert_eq!(tree.max(), None);
        
//         tree.insert(5);

//         assert_eq!(tree.min(), Some(5));
//         assert_eq!(tree.max(), Some(5));

//         tree.insert(2);
//         tree.insert(9);

//         assert_eq!(tree.min(), Some(2));
//         assert_eq!(tree.max(), Some(9));
//     }

//     #[test]
//     fn contain() {
//         let mut tree = FullBinTree::<u64>::new();
        
//         tree.insert(5);
//         tree.insert(2);
//         tree.insert(9);


//         assert!(tree.contains(9), "tree should contain 9");
//         assert!(tree.contains(2), "tree should contain 2");
//         assert!(tree.contains(5), "tree should contain 5");
//         assert!(!tree.contains(4), "tree should not contain 4");
//     }

//     #[test]
//     fn delete() {
        
//         let mut tree = FullBinTree::<u64>::new();
        
//         tree.insert(5);
//         tree.insert(2);
//         tree.insert(9);

//         assert!(tree.contains(5), "tree should contain 5");

//         tree.delete(5);

//         assert!(!tree.contains(5));
//         assert!(tree.contains(9));
//     }
// }