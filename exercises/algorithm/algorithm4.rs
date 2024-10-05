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

    fn dfs_insert(root: &mut Box<TreeNode<T>>, new: Box<TreeNode<T>>) {
        if root.value == new.value {

        }
        else if root.value > new.value {
            match root.left {
                None => root.left = Some(new),
                Some(ref mut lroot) => Self::dfs_insert(lroot, new)
            }
        } else {
            match  root.right {
                None => root.right = Some(new),
                Some(ref mut rroot) => Self::dfs_insert(rroot, new)
            }
        }
    }
    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        let node = Box::new(TreeNode::new(value));
        match self.root {
            None=>self.root=Some(node),
            Some(ref mut root)=>Self::dfs_insert(root, node)
        }
    }

    fn dfs_search(root: &Option<Box<TreeNode<T>>>, value: T) -> bool {
        match root {
            None=>false,
            Some(ref new) => if new.value==value {
                true
            } else if new.value > value {
                Self::dfs_search(&new.left, value)
            } else {
                Self::dfs_search(&new.right, value)
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        return  Self::dfs_search(&self.root, value);
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
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
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


