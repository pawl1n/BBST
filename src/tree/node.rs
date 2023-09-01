use std::cmp::{max, Ordering};
use std::cmp::{max, Ordering, Ordering::*};
use std::fmt::{Debug, Display, Formatter, Result};
use std::mem::swap;

#[derive(Debug, Clone, Eq, Default)]
pub struct Node<T: Ord> {
    pub value: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
    height: u32,
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

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn insert(&mut self, node: Self) {
        // Determine where to insert
        match (*self).cmp(&node) {
            Greater => {
                if self.left.is_none() {
                    self.left = Some(Box::new(node));
                } else {
                    self.left.as_mut().unwrap().insert(node);
                }
            }
            Less => {
                if self.right.is_none() {
                    self.right = Some(Box::new(node));
                } else {
                    self.right.as_mut().unwrap().insert(node);
                }
            }
            Equal => {}
        }

        self.balance();
    }

    pub fn exists(&self, value: T) -> bool {
        match self.value.cmp(&value) {
            Greater => self.left.as_ref().map_or(false, |l| l.exists(value)),
            Less => self.right.as_ref().map_or(false, |r| r.exists(value)),
            Equal => true,
        }
    }

    pub fn delete(&mut self, value: T) {
        match self.value.cmp(&value) {
            Greater => {
                if let Some(left) = self.left.as_mut() {
                    if left.height > 1 {
                        left.delete(value);
                    } else if left.value == value {
                        self.left.take();
                    }
                }
            }
            Less => {
                if let Some(right) = self.right.as_mut() {
                    if right.height > 1 {
                        right.delete(value);
                    } else if right.value == value {
                        self.right.take();
                    }
                }
            }
            Equal => {
                let left_height = self.left.as_ref().map_or(0, |l| l.height);
                let right_height = self.right.as_ref().map_or(0, |r| r.height);

                match left_height.cmp(&right_height) {
                    Less | Ordering::Equal => {
                        let mut right = self.right.take();
                        let left = self.left.take();
                        let mut taken = right.as_mut().unwrap().take_leftmost();
                        if taken.is_none() {
                            swap(self, right.as_mut().unwrap());
                        } else {
                            swap(self, taken.as_mut().unwrap());
                            self.right = right;
                        }
                        self.left = left;
                    }
                    Greater => {
                        let mut left = self.left.take();
                        let right = self.right.take();
                        let mut taken = left.as_mut().unwrap().take_rightmost();
                        if taken.is_none() {
                            swap(self, left.as_mut().unwrap());
                        } else {
                            swap(self, taken.as_mut().unwrap());
                            self.left = left;
                        }
                        self.right = right;
                    }
                }
            }
        }

        self.balance();
    }

    fn balance(&mut self) {
        // Get heights
        let left_height = self.left.as_ref().map_or(0, |l| l.height);
        let right_height = self.right.as_ref().map_or(0, |r| r.height);

        // Update height
        self.calculate_height(left_height, right_height);

        // Determine whether to rotate and direction
        if left_height > right_height + 1 {
            self.rotate_right();
        } else if right_height > left_height + 1 {
            self.rotate_left();
        }
    }

    fn update_height(&mut self) {
        let left_height = self.left.as_ref().map_or(0, |l| l.height);
        let right_height = self.right.as_ref().map_or(0, |r| r.height);

        self.calculate_height(left_height, right_height);
    }

    fn calculate_height(&mut self, left_height: u32, right_height: u32) {
        self.height = 1 + max(left_height, right_height);
    }

    fn take_leftmost(&mut self) -> Option<Box<Self>> {
        if let Some(l) = self.left.as_mut() {
            if l.left.is_some() {
                l.take_leftmost()
            } else {
                let mut taken = self.left.as_mut().unwrap().right.take();
                swap(&mut self.left, &mut taken);

                self.update_height();

                taken
            }
        } else {
            None
        }
    }

    fn take_rightmost(&mut self) -> Option<Box<Self>> {
        if let Some(r) = self.right.as_mut() {
            if r.right.is_some() {
                r.take_rightmost()
            } else {
                let mut taken = self.right.as_mut().unwrap().left.take();
                swap(&mut self.right, &mut taken);

                self.update_height();

                taken
            }
        } else {
            None
        }
    }

    pub fn rotate_right(&mut self) {
        if self.left.is_none() {
            return;
        }

        // 1. Take B
        let mut b: Box<Self> = self.left.take().unwrap();

        // 2. Swap B with A
        swap(self, &mut b);
        let mut a: Box<Self> = b; // Rename variable

        // 3. Take E and make it a left node of A
        a.left = self.right.take();

        // 4. Attach A as right node of B
        a.height -= 2;
        self.right = Some(a);
    }

    pub fn rotate_left(&mut self) {
        if self.right.is_none() {
            return;
        }

        // 1. Take A
        let mut a: Box<Self> = self.right.take().unwrap();

        // 2. Swap A with B
        swap(self, &mut a);
        let mut b: Box<Self> = a; // Rename variable

        // 3. Take D and make it a right node of B
        b.right = self.left.take();

        // 4. Attach B as left node of A
        b.height -= 2;
        self.left = Some(b);
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
