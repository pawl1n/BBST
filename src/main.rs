mod tree;

use crate::tree::node::Node;

fn main() {
    let node_r: Node<i32> = Node::new(6, None, None);
    let node_ll: Node<i32> = Node::new(2, None, None);
    let node_lr: Node<i32> = Node::new(4, None, None);
    let node_l: Node<i32> = Node::new(3, Some(Box::new(node_ll)), Some(Box::new(node_lr)));
    let mut node_root: Node<i32> = Node::new(5, Some(Box::new(node_l)), Some(Box::new(node_r)));
    println!("Hello, world!");

    println!("{:?}", node_root);

    node_root.rotate_right();

    println!("{:?}", node_root);
}
