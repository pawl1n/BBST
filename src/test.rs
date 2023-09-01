#[cfg(test)]
mod tests {
    use crate::tree::Tree;

    fn create_tree() -> Tree<i32> {
        let mut tree = Tree::new();
        tree.insert(10);
        tree.insert(5);
        tree.insert(15);
        tree
    }

    #[test]
    fn test_single_rotation() {
        let tree = create_tree();
        assert_eq!(tree.root.as_ref().unwrap().height, 2);
    }

    #[test]
    fn test_double_rotation() {
        let mut tree = create_tree();
        tree.insert(12);
        tree.insert(17);
        assert_eq!(tree.root.as_ref().unwrap().height, 3);
    }

    #[test]
    fn test_insert_and_delete() {
        let mut tree = create_tree();
        tree.delete(5);
        assert_eq!(tree.root.as_ref().unwrap().height, 2);
    }

    #[test]
    fn test_insert_and_delete_multiple() {
        let mut tree = Tree::new();
        tree.insert(10);
        tree.insert(5);
        tree.insert(15);
        tree.insert(3);
        tree.delete(15);
        tree.delete(5);
        assert_eq!(tree.root.as_ref().unwrap().height, 2);
    }

    #[test]
    fn test_insert_delete_balancing() {
        let mut tree = create_tree();
        tree.insert(7);
        tree.insert(12);
        tree.delete(5);
        assert_eq!(tree.root.as_ref().unwrap().height, 3);
    }

    #[test]
    fn test_exists() {
        let mut tree = create_tree();
        assert_eq!(tree.exists(5), true);
        assert_eq!(tree.exists(7), false);
        tree.insert(7);
        assert_eq!(tree.exists(7), true);
        tree.delete(7);
        assert_eq!(tree.exists(7), false);
    }

    #[test]
    fn test_empty_tree() {
        let tree: Tree<i32> = Tree::new();
        assert_eq!(tree.root.is_none(), true);
    }

    #[test]
    fn test_insert_many() {
        let mut tree = Tree::new();
        for i in 1..=100 {
            tree.insert(i);
        }
        for i in 1..=100 {
            assert_eq!(tree.exists(i), true);
        }
        assert_eq!(tree.root.as_ref().unwrap().height, 7); // AVL tree should be balanced
    }

    #[test]
    fn test_delete_nonexistent() {
        let mut tree = create_tree();
        tree.delete(7); // 7 is not in the tree
        assert_eq!(tree.root.as_ref().unwrap().height, 2); // Height should remain unchanged
    }

    #[test]
    fn test_delete_root() {
        let mut tree = create_tree();
        tree.delete(10);
        assert_eq!(tree.root.as_ref().unwrap().value, 15); // 15 becomes the new root
    }

    #[test]
    fn test_multiple_rotations() {
        let mut tree = Tree::new();
        tree.insert(10);
        tree.insert(20);
        tree.insert(5);
        tree.insert(25);
        tree.insert(15);
        tree.insert(30);
        tree.insert(35);
        tree.insert(40);
        assert_eq!(tree.root.as_ref().unwrap().height, 4); // AVL tree should be balanced
    }

    #[test]
    fn test_delete_leaf_node() {
        let mut tree = create_tree();
        tree.delete(15);
        assert_eq!(tree.exists(15), false);
        assert_eq!(tree.root.as_ref().unwrap().height, 2);
    }

    #[test]
    fn test_delete_node_with_one_child() {
        let mut tree = create_tree();
        tree.delete(5);
        assert_eq!(tree.exists(5), false);
        assert_eq!(tree.root.as_ref().unwrap().height, 2);
    }

    #[test]
    fn test_delete_node_with_two_children() {
        let mut tree = Tree::new();
        tree.insert(10);
        tree.insert(5);
        tree.insert(15);
        tree.insert(3);
        tree.insert(7);
        tree.insert(12);
        tree.insert(17);
        tree.delete(10);
        assert_eq!(tree.exists(10), false);
        assert_eq!(tree.root.as_ref().unwrap().height, 3);
    }

    #[test]
    fn test_delete_root_with_two_children() {
        let mut tree = Tree::new();
        tree.insert(10);
        tree.insert(5);
        tree.insert(15);
        tree.delete(10);
        assert_eq!(tree.exists(10), false);
        assert_eq!(tree.root.as_ref().unwrap().value, 15);
    }

    #[test]
    fn test_delete_nonexistent_node() {
        let mut tree = create_tree();
        tree.delete(7); // 7 is not in the tree
        assert_eq!(tree.root.as_ref().unwrap().height, 2); // Height should remain unchanged
    }

    #[test]
    fn test_delete_in_large_tree() {
        let mut tree = Tree::new();

        // Insert 100 elements into the tree
        for i in 1..=100 {
            tree.insert(i);
        }

        // Delete some nodes and check the tree's height
        tree.delete(50);
        tree.delete(25);
        tree.delete(75);

        // Verify that the deleted nodes no longer exist
        assert_eq!(tree.exists(50), false);
        assert_eq!(tree.exists(25), false);
        assert_eq!(tree.exists(75), false);

        for i in 1..=100 {
            if i != 50 && i != 25 && i != 75 {
                assert_eq!(tree.exists(i), true);
            }
        }

        // Check that the tree's height is still relatively balanced
        assert_eq!(tree.root.as_ref().unwrap().height, 7); // Log₂(100) ≈ 6.64
    }

    #[test]
    fn test_delete_taking_rightmost() {
        let mut tree = Tree::new();

        tree.insert(5);
        tree.insert(6);
        tree.insert(2);
        tree.insert(3);
        tree.insert(1);
        tree.insert(7);
        tree.insert(4);

        tree.delete(5);

        for i in 1..=7 {
            if i != 5 {
                assert_eq!(tree.exists(i), true);
            } else {
                assert_eq!(tree.exists(i), false);
            }
        }
    }

    #[test]
    fn test_delete_taking_left() {
        let mut tree = Tree::new();

        tree.insert(2);
        tree.insert(1);

        tree.delete(2);

        assert!(!tree.exists(2));
        assert!(tree.exists(1));
    }

    #[test]
    fn test_delete_when_only_node() {
        let mut tree = Tree::new();
        tree.insert(1);
        tree.delete(1);
        assert!(!tree.exists(1));
    }
}
