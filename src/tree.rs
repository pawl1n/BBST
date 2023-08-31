mod node;

use crate::tree::node::Node;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug, Clone, Eq)]
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

        println!("{:?}", self.root);
    }

    pub fn exists(&self, value: T) -> bool {
        self.root.as_ref().map_or(false, |n| n.exists(value))
    }

    pub fn delete(&mut self, value: T) {
        if self.root.is_some() {
            if self.root.as_ref().unwrap().height > 1 {
                self.root.as_mut().unwrap().delete(value);
            } else {
                if self.root.as_ref().unwrap().value == value {
                    self.root.take();
                }
            }
        }

        println!("{:?}", self.root);
    }
}

impl<T: Display + Ord + Debug> Display for Tree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.root)
    }
}

impl<T: Ord> PartialOrd for Tree<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> Ord for Tree<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.root.cmp(&other.root)
    }
}

impl<T: Ord> PartialEq for Tree<T> {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root
    }
}
