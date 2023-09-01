mod test;
mod tree;

use crate::tree::Tree;

fn main() {
    let mut input = String::new();
    let mut tree: Tree<i32> = Tree::new();

    while let Ok(_) = std::io::stdin().read_line(&mut input) {
        if input.is_empty() {
            break;
        }

        let v: Vec<&str> = input.split(" ").collect();
        let n: i32 = v[1].trim().parse().unwrap();

        match v[0] {
            "insert" => tree.insert(n),
            "delete" => tree.delete(n),
            "exists" => println!("{}", tree.exists(n)),
            _ => (),
        }

        input.clear();
    }
}
