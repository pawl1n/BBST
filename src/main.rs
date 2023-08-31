mod tree;

use crate::tree::Tree;

fn main() {
    let mut tree = Tree::<i32>::new();

    tree.insert(1);
    tree.insert(2);
    tree.insert(3);
    tree.insert(4);
    tree.insert(5);
    tree.insert(6);

    tree.delete(2);
    tree.delete(5);

    let res = tree.exists(6);
    println!("{}", res);
}
