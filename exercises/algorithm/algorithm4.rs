/*
    binary_search tree
    This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        if self.root.is_none() {
            let node = Box::new(TreeNode::new(value));
            self.root = Some(node);
        } else {
            self.root.as_mut().unwrap().insert(value);
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        let mut root_node = &self.root;
        if root_node.is_none() {
            return false;
        }

        loop {
            match root_node.as_ref().unwrap().value.cmp(&value) {
                Ordering::Greater => {
                    if root_node.as_ref().unwrap().left.is_none() {
                        return false;
                    } else {
                        root_node = &root_node.as_ref().unwrap().left;
                    }
                }
                Ordering::Less => {
                    if root_node.as_ref().unwrap().right.is_none() {
                        return false;
                    } else {
                        root_node = &root_node.as_ref().unwrap().right;
                    }
                }
                Ordering::Equal => {
                    return true;
                }
            }
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        match self.value.cmp(&value) {
            Ordering::Greater => {
                if self.left.is_none() {
                    let left = Box::new(TreeNode::new(value));
                    self.left = Some(left)
                } else {
                    self.left.as_mut().unwrap().insert(value);
                }
            }
            Ordering::Less => {
                if self.right.is_none() {
                    let right = Box::new(TreeNode::new(value));
                    self.right = Some(right)
                } else {
                    self.right.as_mut().unwrap().insert(value);
                }
            }

            Ordering::Equal => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
