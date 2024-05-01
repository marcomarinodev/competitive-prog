#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let mut tree = Tree::with_root(10);
        assert_eq!(tree.sum(), 10);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2
        assert_eq!(tree.sum(), 37);

        tree.add_node(1, 7, false); // id 3
        tree.add_node(2, 20, true); // id 4
        assert_eq!(tree.sum(), 64);
    }

    #[test]
    fn test_is_bst() {
        let mut tree = Tree::with_root(5);

        tree.add_node(0, 3, true);
        tree.add_node(0, 7, false);
        assert_eq!(tree.is_bst(), true);

        tree.add_node(1, 1, true);
        tree.add_node(1, 4, false);
        assert_eq!(tree.is_bst(), true);

        tree.add_node(2, 6, true);
        tree.add_node(2, 8, false);
        assert_eq!(tree.is_bst(), true);

        tree.add_node(6, 6, false);
        assert_eq!(tree.is_bst(), false);
    }

    #[test]
    fn test_is_balanced() {
        let mut tree = Tree::with_root(1);
        assert_eq!(tree.is_balanced(), true);

        tree.add_node(0, 2, true); // id 1
        assert_eq!(tree.is_balanced(), true);

        tree.add_node(1, 3, true); // id 2
        assert_eq!(tree.is_balanced(), false);

        tree.add_node(2, 6, true); // id 3
        assert_eq!(tree.is_balanced(), false);

        tree.add_node(0, 4, false); // id 4
        assert_eq!(tree.is_balanced(), false);

        tree.add_node(4, 5, false); // id 5
        assert_eq!(tree.is_balanced(), false);

        tree.add_node(5, 7, false); // id 6
        assert_eq!(tree.is_balanced(), false);

        tree.add_node(1, 8, false); // id 7 
        assert_eq!(tree.is_balanced(), false);

        tree.add_node(4, 9, true); // id 8
        assert_eq!(tree.is_balanced(), true);

    }

    #[test]
    fn test_is_complete() {
        let mut tree = Tree::with_root(1);
        assert_eq!(tree.verify_completeness_property(), true);

        tree.add_node(0, 3, false); // id 1
        assert_eq!(tree.verify_completeness_property(), false);

        tree.add_node(0, 2, true); // id 2
        assert_eq!(tree.verify_completeness_property(), true);

        tree.add_node(2, 4, true); // id 3
        tree.add_node(2, 5, false); // id 4
        assert_eq!(tree.verify_completeness_property(), true);

        tree.add_node(1, 7, false); // id 5
        assert_eq!(tree.verify_completeness_property(), false);

        tree.add_node(1, 6, true); // id 6
        assert_eq!(tree.verify_completeness_property(), true);

        tree.add_node(3, 8, true); // id 7
        assert_eq!(tree.verify_completeness_property(), true);

        tree.add_node(7, 10, false); // id 8
        assert_eq!(tree.verify_completeness_property(), false);
    }

    #[test]
    fn test_is_max_heap() {
        // ==> invalid max heap property
        let mut tree = Tree::with_root(10);
        assert_eq!(tree.is_max_heap(), true);

        tree.add_node(0, 7, true); // id 1
        tree.add_node(0, 8, false); // id 2
        assert_eq!(tree.is_max_heap(), true);

        tree.add_node(1, 9, true); // id 3
        assert_eq!(tree.is_max_heap(), false);

        // uncompleted tree
        let mut tree = Tree::with_root(10);
        assert_eq!(tree.is_max_heap(), true);

        tree.add_node(0, 7, true); // id 1
        tree.add_node(0, 8, false); // id 2
        assert_eq!(tree.is_max_heap(), true);

        tree.add_node(1, 6, true); // id 3
        assert_eq!(tree.is_max_heap(), true);

        tree.add_node(2, 6, true); // id 4
        assert_eq!(tree.is_max_heap(), false);
    }
}