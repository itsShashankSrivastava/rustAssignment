//Given a binary tree, implement a function that returns the maximum depth of the tree.

use std::collections::VecDeque;

// Node struct for the binary tree
#[derive(Debug)]
struct Node {
    data: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

// Implementation of the Node struct
impl Node {
    // Constructor to initialize
    // the node with a value
    fn new(val: i32) -> Self {
        Node {
            data: val,
            left: None,
            right: None,
        }
    }
}

// Solution struct
struct Solution;

// Implementation of the Solution struct
impl Solution {
    // Function to find the maximum depth of a binary tree
    // using level order traversal
    fn max_depth(root: Option<Box<Node>>) -> i32 {
        // If the root is None
        // (empty tree), depth is 0
        if root.is_none() {
            return 0;
        }

        // Create a queue for
        // level order traversal
        let mut queue = VecDeque::new();
        let mut level = 0;

        // Push the root node into the queue
        queue.push_back(root);

        // While there are nodes in the queue
        while !queue.is_empty() {
            // Get the number of nodes
            // at the current level
            let size = queue.len();

            // Process all nodes
            // at the current level
            for _ in 0..size {
                // Get the front node in the queue
                if let Some(mut front) = queue.pop_front().flatten() {
                    // Enqueue left child if exists
                    if let Some(left) = front.left.take() {
                        queue.push_back(Some(left));
                    }

                    // Enqueue right child if exists
                    if let Some(right) = front.right.take() {
                        queue.push_back(Some(right));
                    }
                }
            }
            // Increment level to
            // move to the next level
            level += 1;
        }
        // Return the level, which represents
        // the maximum depth of the tree
        level
    }
}

// Main function
fn main() {
    // Creating a sample binary tree
    let root = Some(Box::new(Node {
        data: 1,
        left: Some(Box::new(Node {
            data: 2,
            left: Some(Box::new(Node::new(4))),
            right: Some(Box::new(Node {
                data: 5,
                left: None,
                right: Some(Box::new(Node {
                    data: 6,
                    left: None,
                    right: Some(Box::new(Node::new(7))),
                })),
            })),
        })),
        right: Some(Box::new(Node::new(3))),
    }));

    // Find the maximum depth of the binary tree
    let depth = Solution::max_depth(root);

    println!("Maximum depth of the binary tree: {}", depth);
}
