
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

impl<T> Node<T>
where T: Copy + Eq + PartialOrd {
    fn min_node(&self) -> &Node<T> {
        match self.left.as_ref() {
            None => self,
            Some(left) => left.min_node()
        }
    }

    fn max_node(&self) -> &Node<T> {
        match self.right.as_ref() {
            None => self,
            Some(right) => right.max_node()
        }
    }

    fn contains(&self, elem : &T) -> bool {
        if self.value == *elem {
            return true;
        }
        let node = if self.value < *elem {&self.right} else {&self.left};
        match node {
            None => false,
            Some (child) => child.contains(elem)
        }
    }

    fn insert_here(&mut self, elem : T) {
        let new_node = Some(Box::new(Node {
                value : elem,
                left : None,
                right : None
        }));
        if self.value < elem {
            self.right = new_node;
        }
        else {
            self.left = new_node;
        }
    }

    fn insert_find(&mut self, elem : T) {
        let node = if self.value < elem {&mut self.right} else {&mut self.left};
        match node {
            None => self.insert_here(elem),
            Some(child) => child.insert_find(elem)
        }
    }
}


impl<T> BinTree<T> 
where T: Copy + Eq + PartialOrd {
    pub fn new() -> Self {
        BinTree { root : None}
    }

    pub fn min(&self) -> Option<&Node<T>> {
        match self.root.as_ref() {
            None => None,
            Some(root_node) => Some(root_node.min_node())
        }
    }

    pub fn max(&self) -> Option<&Node<T>> {
        match self.root.as_ref() {
            None => None,
            Some(root_node) => Some(root_node.max_node())
        }
    }

    pub fn insert(&mut self, elem : T) {
        match &mut self.root {
            None => {
                let new_node = Some(Box::new(Node {
                    value : elem,
                    left : None,
                    right : None
                }));
                self.root = new_node;
            },
            Some(root) => root.insert_find(elem),
        }        
    }

    pub fn contains(&self, elem : &T) -> bool {
        match &self.root {
            None => false,
            Some(node) => node.contains(elem)
        }
    }

    pub fn delete(&mut self, value : T) {
        panic!("Not Implemented");
    }
}   


impl<T> Drop for BinTree<T>
where T:Copy + Eq + PartialOrd {
    fn drop(&mut self) {
        self.root.take();
    }
}

impl<T> Drop for Node<T> 
where T:Copy + Eq + PartialOrd {
    fn drop(&mut self) {
        self.left.take();
        self.right.take();
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn min_max () {
        let mut tree: BinTree<u64> = BinTree::<u64>::new();

        assert!(tree.min().is_none());
        assert!(tree.max().is_none());
        
        tree.insert(5);

        assert_eq!(tree.min().unwrap().value, 5);
        assert_eq!(tree.max().unwrap().value, 5);

        tree.insert(2);
        tree.insert(9);

        assert_eq!(tree.min().unwrap().value, 2);
        assert_eq!(tree.max().unwrap().value, 9);

        tree.insert(1);
        tree.insert(3);
        tree.insert(7);
        tree.insert(10);

        assert_eq!(tree.min().unwrap().value, 1);
        assert_eq!(tree.max().unwrap().value, 10);

    }

    #[test]
    fn contain() {
        let mut tree = BinTree::<u64>::new();
        
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
        
        let mut tree = BinTree::<u64>::new();
        
        tree.insert(5);
        tree.insert(2);
        tree.insert(9);

        assert!(tree.contains(&5), "tree should contain 5");

        tree.delete(5);

        assert!(!tree.contains(&5));
        assert!(tree.contains(&9));
    }
}