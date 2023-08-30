use crate::tree::node::Node;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug, Clone, Eq)]
pub struct Tree<T: Ord> {
    pub root: Option<Box<Node<T>>>,
}

#[allow(dead_code)]
impl<T: Ord> Tree<T> {
    pub fn new() -> Self {
        Self { root: None }
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
