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
        //TODO
        match &mut self.root {
            // Case 1: 树是空的 (root is None)
            // 我们创建一个新的 TreeNode，用 Box 包装它，然后设置为根节点。
            None => {
                self.root = Some(Box::new(TreeNode::new(value)));
            }
            // Case 2: 树非空 (root is Some)
            // 我们调用 TreeNode 自己的 insert 方法，让它递归地找到插入位置。
            Some(node) => {
                node.insert(value);
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        // 从根节点开始迭代搜索
        let mut current_node = &self.root;
        // 使用 `while let` 循环，只要当前节点不是 None，就继续
        while let Some(node) = current_node {
            match value.cmp(&node.value) {
                // 如果要找的值小于当前节点的值，则往左子树走
                Ordering::Less => {
                    current_node = &node.left;
                }
                // 如果要找的值大于当前节点的值，则往右子树走
                Ordering::Greater => {
                    current_node = &node.right;
                }
                // 如果值相等，说明找到了，返回 true
                Ordering::Equal => {
                    return true;
                }
            }
        }
        // 如果循环结束（意味着我们走到了一个 None 的位置），说明没找到，返回 false
        false
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
             match value.cmp(&self.value) {
            // Case 1: 新值更小，应该插入到左子树
            Ordering::Less => {
                // 检查左子节点是否存在
                match &mut self.left {
                    // 如果左子节点为空，就在这里创建新节点
                    None => {
                        self.left = Some(Box::new(TreeNode::new(value)));
                    }
                    // 如果左子节点已存在，就递归调用 insert
                    Some(node) => {
                        node.insert(value);
                    }
                }
            }
            // Case 2: 新值更大，应该插入到右子树
            Ordering::Greater => {
                // 检查右子节点是否存在
                match &mut self.right {
                    // 如果右子节点为空，就在这里创建新节点
                    None => {
                        self.right = Some(Box::new(TreeNode::new(value)));
                    }
                    // 如果右子节点已存在，就递归调用 insert
                    Some(node) => {
                        node.insert(value);
                    }
                }
            }
            // Case 3: 值相等。在标准的 BST 中，我们通常不插入重复值。
            // 所以这里什么也不做。
            Ordering::Equal => {
                // Do nothing
            }
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
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


