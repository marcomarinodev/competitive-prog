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

    pub fn is_max_heap(&self) -> bool {
        self.verify_max_heap_property(Some(0)) && self.verify_completeness_property()
    }

    fn verify_max_heap_property(&self, node_id: Option<usize>) -> bool {
        if let Some(id) = node_id {
            let node = &self.nodes[id];

            if let Some(left_id) = node.id_left {
                if node.key < *&self.nodes[left_id].key {
                    return false;
                }
            }

            if let Some(right_id) = node.id_right {
                if node.key < *&self.nodes[right_id].key {
                    return false;
                }
            }

            let left_ok = self.verify_max_heap_property(node.id_left);
            let right_ok = self.verify_max_heap_property(node.id_right);

            return left_ok && right_ok;
        }

        true
    }

    /// if i encounter a non full node, then the next one 
    /// shouldn't have any child, otherwise is not complete
    fn verify_completeness_property(&self) -> bool {
        let mut to_visit: Vec<usize> = Vec::new();
        let mut flag = false;

        
        to_visit.push(0);

        while !to_visit.is_empty() {
            let node = &self.nodes[to_visit.remove(0)];

            if let Some(id_left) = node.id_left {
                if flag { return false }

                to_visit.push(id_left);
            } else { flag = true }

            if let Some(id_right) = node.id_right {
                if flag { return false }

                to_visit.push(id_right);
            } else { flag = true }
        }
        

        true
    }

}

