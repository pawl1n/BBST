#[cfg(test)]
mod test {
    use crate::tree::Tree;

    #[test]
    fn test_insert() {
        let mut tree: Tree<i32> = Tree::new();

        tree.insert(1);

        assert_eq!(tree.root.as_ref().unwrap().value, 1);
    }

    #[test]
    fn test_exists() {
        let mut tree: Tree<i32> = Tree::new();

        tree.insert(1);

        assert!(tree.exists(1));
    }

    #[test]
    fn test_delete() {
        let mut tree: Tree<i32> = Tree::new();

        tree.insert(1);
        tree.delete(1);

        assert!(!tree.exists(1));
    }

    #[test]
    fn test_height() {
        let mut tree: Tree<i32> = Tree::new();

        for i in 0..=100 {
            tree.insert(i);
        }

        assert_eq!(tree.root.as_ref().unwrap().height(), 7);
    }

    #[test]
    fn test_right_rotation() {
        let mut tree: Tree<i32> = Tree::new();

        for i in (0..=10).rev() {
            tree.insert(i);
        }

        for i in 0..=10 {
            assert!(tree.exists(i));
        }
    }

    #[test]
    fn test_left_rotation() {
        let mut tree: Tree<i32> = Tree::new();

        for i in 0..=10 {
            tree.insert(i);
        }

        for i in 0..=10 {
            assert!(tree.exists(i));
        }
    }

    #[test]
    fn test_take_leftmost() {
        let mut tree: Tree<i32> = Tree::new();

        tree.insert(5);
        tree.insert(3);
        tree.insert(9);
        tree.insert(1);
        tree.insert(4);
        tree.insert(7);
        tree.insert(10);
        tree.insert(8);

        assert_eq!(tree.root.as_ref().unwrap().value, 5);

        tree.delete(5);

        assert!(!tree.exists(5));
        assert!(tree.exists(3));
        assert!(tree.exists(9));
        assert!(tree.exists(1));
        assert!(tree.exists(4));
        assert!(tree.exists(7));
        assert!(tree.exists(10));
        assert!(tree.exists(8));

        assert_eq!(tree.root.as_ref().unwrap().height(), 3);
    }

    #[test]
    fn test_take_rightmost() {
        let mut tree: Tree<i32> = Tree::new();

        tree.insert(5);
        tree.insert(2);
        tree.insert(9);
        tree.insert(1);
        tree.insert(4);
        tree.insert(7);
        tree.insert(8);
        tree.insert(3);

        assert_eq!(tree.root.as_ref().unwrap().value, 5);

        tree.delete(5);

        assert!(!tree.exists(5));
        assert!(tree.exists(2));
        assert!(tree.exists(9));
        assert!(tree.exists(1));
        assert!(tree.exists(4));
        assert!(tree.exists(7));
        assert!(tree.exists(8));
        assert!(tree.exists(3));

        assert_eq!(tree.root.as_ref().unwrap().height(), 3);
    }

    #[test]
    fn test_height_after_left_rotation() {
        let mut tree: Tree<i32> = Tree::new();

        tree.insert(3);
        tree.insert(1);
        tree.insert(2);

        assert_eq!(tree.root.as_ref().unwrap().height(), 2);
    }

    #[test]
    fn test_remove_unexisting_right() {
        let mut tree: Tree<i32> = Tree::new();

        tree.insert(1);
        tree.insert(2);

        tree.delete(3);

        assert!(tree.exists(1));
        assert!(tree.exists(2));
    }

    #[test]
    fn test_remove_unexisting_left() {
        let mut tree: Tree<i32> = Tree::new();

        tree.insert(3);
        tree.insert(2);

        tree.delete(1);

        assert!(tree.exists(3));
        assert!(tree.exists(2));
    }

    #[test]
    fn test_insert_many() {
        let mut tree: Tree<i32> = Tree::new();

        for i in 0..=100 {
            tree.insert(i);
        }

        for i in (101..=200).rev() {
            tree.insert(i);
        }

        assert_eq!(tree.root.as_ref().unwrap().height(), 8);

        for i in 0..=200 {
            assert!(tree.exists(i));
        }
    }

    #[test]
    fn test_delete_many() {
        let mut tree: Tree<i32> = Tree::new();

        for i in 0..=100 {
            tree.insert(i);
        }

        tree.delete(101);
        tree.delete(50);
        tree.delete(25);
        tree.delete(75);

        assert_eq!(tree.root.as_ref().unwrap().height(), 7);

        for i in 0..=100 {
            if i != 101 && i != 50 && i != 25 && i != 75 {
                assert!(tree.exists(i));
            }
        }
    }
}
