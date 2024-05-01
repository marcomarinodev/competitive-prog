// basic bin tree implementation (on array)
struct Node {
    key: u32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    fn new(key: u32) -> Self {
        Self {
            key, 
            id_left: None,
            id_right: None,
        }
    }
}

struct Tree {
    nodes: Vec<Node>
}

impl Tree {
    pub fn with_root(key: u32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    /// Adds a child to the node with `parent_id` and returns the id of the new node. 
    /// The new node has the specified `key`. The new node is the left  child of the  
    /// node `parent_id` iff `is_left` is `true`, the right child otherwise.
    ///
    /// # Panics
    /// Panics if the `parent_id` does not exist, or if the node `parent_id ` has  
    /// the child already set.
    pub fn add_node(&mut self, parent_id: usize, key: u32, is_left: bool) -> usize {
        assert!(
            parent_id < self.nodes.len(),
            "Parent node id does not exist"
        );
        if is_left {
            assert!(
                self.nodes[parent_id].id_left == None,
                "Parent node has the left child already set"
            );
        } else {
            assert!(
                self.nodes[parent_id].id_right == None,
                "Parent node has the right child already set"
            );
        }

        // child position in array
        let child_id = self.nodes.len();
        self.nodes.push(Node::new(key));

        let child = if is_left {
            &mut self.nodes[parent_id].id_left
        } else {
            &mut self.nodes[parent_id].id_right
        };

        // parent.id_left or parent.id_right = child_id
        *child = Some(child_id);

        child_id
    }

    /// Returns the sum of all the keys in the tree
    pub fn sum(&self) -> u32 {
        self.rec_sum(Some(0))
    }

    /// A private recursive function that computes the sum of 
    /// nodes in the subtree rooted at `node_id`
    fn rec_sum(&self, node_id: Option<usize>) -> u32 {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            let left_sum = self.rec_sum(node.id_left);
            let right_sum = self.rec_sum(node.id_right);

            return left_sum + right_sum + node.key;
        }

        0
    }

    /// A method to check if the binary tree is a Binary Search Tree
    pub fn is_bst(&self) -> bool {
        self.rec_is_bst(Some(0))
    }
    
    fn rec_is_bst(&self, node_id: Option<usize>) -> bool {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            if let Some(id_left) = node.id_left {
                let left_node = &self.nodes[id_left];

                if left_node.key > node.key {
                    return false;
                }
            }

            if let Some(id_right) = node.id_right {
                let right_node = &self.nodes[id_right];

                if right_node.key <= node.key {
                    return false;
                }
            }

            return self.rec_is_bst(node.id_left) && self.rec_is_bst(node.id_right);   
        }

        true
    }

    pub fn is_balanced(&self) -> bool {
        let mut height = 0;
        self.rec_is_balanced(Some(0), &mut height)
    }

    fn rec_is_balanced(&self, node_id: Option<usize>, height: &mut i32) -> bool {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            let mut left_height = 0;
            let mut right_height = 0;
            
            let left_ok = if node.id_left == None { true } else { 
                self.rec_is_balanced(node.id_left, &mut left_height)
            };

            let right_ok = if node.id_right == None { true } else { 
                self.rec_is_balanced(node.id_right, &mut right_height)
            };

            // the height of the current node is the maximum height between left and right
            // plus the current node itself
            *height = left_height.max(right_height) + 1;

            if (left_height - right_height).abs() > 1 {
                return  false;
            }

            return left_ok && right_ok;
        }

        true
    }

}

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
}