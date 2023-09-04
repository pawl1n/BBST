mod node;

use crate::tree::node::Node;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug, Clone)]
pub struct Tree<T: Ord> {
    pub root: Option<Box<Node<T>>>,
}

#[allow(dead_code)]
impl<T: Ord + Default + Display + Debug> Tree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, value: T) {
        let node = Node::new(value, None, None);

        if self.root.is_none() {
            self.root = Some(Box::new(node));
        } else {
            self.root.as_mut().unwrap().insert(node);
        }

        // println!("{:?}", self.root);
    }

    pub fn exists(&self, value: T) -> bool {
        self.root.as_ref().map_or(false, |n| n.exists(value))
    }

    pub fn delete(&mut self, value: T) {
        if let Some(root) = self.root.as_mut() {
            if root.height() == 1 && root.value == value {
                self.root = None;
            } else {
                root.delete(value);
            }
        }

        // println!("{:?}", self.root);
    }
}

impl<T: Display + Ord + Debug> Display for Tree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.root)
    }
}
