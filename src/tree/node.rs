use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter, Result};
use std::mem::swap;

#[derive(Debug, Clone, Eq, Default)]
pub struct Node<T: Ord> {
    pub value: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
    pub height: u32,
}

#[allow(dead_code)] // TODO: Remove after writing tests
impl<T: Ord + Default + Display + Debug> Node<T> {
    pub fn new(value: T, left: Option<Box<Node<T>>>, right: Option<Box<Node<T>>>) -> Self {
        Self {
            value,
            left,
            right,
            height: 1,
        }
    }

    pub fn rotate_right(&mut self) {
        if self.left.is_none() {
            return;
        }

        // 1. Take B
        let mut b: Box<Self> = self.left.take().unwrap();
        // println!("{:?}", self);

        // 2. Swap B with A
        swap(self, &mut b);
        let mut a: Box<Self> = b; // Rename variable

        // 3. Take E and make it a left node of A
        a.left = self.right.take();

        // 4. Attach A as right node of B
        a.height -= 1;
        self.right = Some(a);

        self.height += 1;
    }

    pub fn rotate_left(&mut self) {
        if self.right.is_none() {
            return;
        }

        // 1. Take A
        let mut a: Box<Self> = self.right.take().unwrap();
        // println!("{:?}", self);

        // 2. Swap A with B
        swap(self, &mut a);
        let mut b: Box<Self> = a; // Rename variable

        // 3. Take D and make it a right node of B
        b.right = self.left.take();

        // 4. Attach B as left node of A
        b.height -= 1;
        self.left = Some(b);

        self.height += 1;
    }
}

impl<T: Display + Ord> Display for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
}

impl<T: Ord> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl<T: Ord> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
