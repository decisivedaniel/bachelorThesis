
#[derive(PartialEq)]
pub struct Node<T>
where T: Copy + Eq + PartialOrd {
    left : Option<Box<Node<T>>>,
    right : Option<Box<Node<T>>>,
    value : T
}

pub struct BinTree<T>
where T: Copy + Eq + PartialOrd {
    root : Option<Box<Node<T>>>
}

impl<T> BinTree<T> 
where T: Copy + Eq + PartialOrd {
    pub fn new() -> Self {
        BinTree { root : None}
    }

    pub fn min(&self) -> Option<T> {
        panic!("Not Implemented");
    }

    pub fn max(&self) -> Option<T> {
        panic!("Not Implemented");
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
                println!("Broke before reaching the end");
                break;
            }
        }
        let new_node = Some(Box::new(Node {
                value : value,
                left : None,
                right : None
        }));
        if current.is_none() {
            println!("Set root");
            self.root = new_node
        }
        else if current.as_ref().unwrap().value < value {
            println!("Set right node");
            let left = current.as_mut().unwrap().left.take();
            current.replace(Box::new(Node {
                value : current.as_ref().unwrap().value,
                left : left,
                right : new_node
            }));
        }
        else {
            println!("Set left node");
            let right = current.as_mut().unwrap().right.take();
            current.replace(Box::new(Node {
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
    fn basic() {
        let mut tree = BinTree::<u64>::new();
        tree.insert(5);
        tree.insert(2);
        tree.insert(9);

        //assert_eq!(tree.max(), Some(9));
        //assert_eq!(tree.min(), Some(2));

        assert!(tree.contains(9), "tree should contain 9");
        assert!(tree.contains(2), "tree should contain 2");
        assert!(tree.contains(5), "tree should contain 5");
        assert!(!tree.contains(4), "tree should not contain 4");

        //tree.delete(5);

        //assert_eq!(tree.contains(5), false);
    }
}