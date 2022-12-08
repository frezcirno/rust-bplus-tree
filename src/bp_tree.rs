use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use crate::node::{BPIndexNode, BPLeafNode};
use crate::node::{BPNode, BPNodePtr};

pub struct BPTree<const FANOUT: usize, K: Copy + Ord + Debug, V: Clone + Debug> {
    pub(crate) root: BPNodePtr<FANOUT, K, V>,
}

impl<const FANOUT: usize, K: Copy + Ord + Debug, V: Clone + Debug> Debug for BPTree<FANOUT, K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let node = &self.root;
        let mut queue = VecDeque::new();
        queue.push_back(node.clone());
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            match node.borrow().deref() {
                BPNode::Leaf(leaf) => {
                    f.write_fmt(format_args!("{:?}\n", leaf))?;
                }
                BPNode::Index(index) => {
                    f.write_fmt(format_args!("{:?}\n", index))?;
                    for child in index.get_children() {
                        queue.push_back(child.root.clone());
                    }
                }
            };
        }
        Ok(())
    }
}

impl<const FANOUT: usize, K: Copy + Ord + Debug, V: Clone + Debug> BPTree<FANOUT, K, V> {
    pub fn new() -> Self {
        BPTree {
            root: Rc::new(RefCell::new(BPNode::Leaf(BPLeafNode::new()))),
        }
    }

    pub fn new_from(root: BPNodePtr<FANOUT, K, V>) -> Self {
        BPTree { root }
    }

    fn root_replace(&mut self, new_root: BPNodePtr<FANOUT, K, V>) -> BPNodePtr<FANOUT, K, V> {
        std::mem::replace(&mut self.root, new_root)
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.insert_recur(key, value);
        if self.root.borrow().deref().is_full() {
            let old_root = self.root_replace(BPNode::new_index_ptr());
            let (split_key, right) = BPTree::split_node(&old_root);
            let mut root = self.root.borrow_mut();
            let root = root.as_index_mut();
            root.push_key(split_key);
            root.push_child(BPTree { root: old_root });
            root.push_child(BPTree { root: right });
        }
    }

    fn insert_recur(&mut self, key: K, value: V) {
        let mut root = self.root.borrow_mut();

        if root.is_empty() {
            root.as_leaf_mut().push_key_value(key, value);
            return;
        }

        let index = root.search_key(&key);
        if let Ok(_) = index {
            // If the key is already in the tree, do nothing.
            return;
        } else if let Err(index) = index {
            // If the key is not in the tree
            match root.deref_mut() {
                BPNode::Leaf(lroot) => {
                    lroot.insert_key_value(index, key, value);
                }
                BPNode::Index(iroot) => {
                    let child = iroot.get_child_mut(index).unwrap();
                    child.insert_recur(key, value);
                    if child.root.borrow().is_full() {
                        let (split_key, right) = BPTree::split_node(&child.root);
                        iroot.insert_key_at(index, split_key);
                        iroot.insert_child_at(index + 1, BPTree { root: right });
                    }
                }
            }
        }
    }

    pub fn split_node(node: &BPNodePtr<FANOUT, K, V>) -> (K, BPNodePtr<FANOUT, K, V>) {
        match node.borrow_mut().deref_mut() {
            BPNode::Leaf(leaf) => BPLeafNode::split_leaf_node(node, leaf),
            BPNode::Index(index) => BPIndexNode::split_node(index),
        }
    }

    pub fn remove(&mut self, key: &K) {
        self.remove_recur(key);
        if self.root.borrow().is_empty() {
            let child = self.root.borrow_mut().as_index_mut().remove_child(0);
            self.root_replace(child.root);
        }
    }

    fn remove_recur(&mut self, key: &K) {
        let mut root = self.root.borrow_mut();

        if root.is_empty() {
            return;
        }

        // If the root is a leaf node, just remove the key if exists and return
        if let BPNode::Leaf(leaf) = root.deref_mut() {
            if let Ok(index) = leaf.search_key(&key) {
                leaf.remove(index).unwrap();
            }
            return;
        }

        let root = root.as_index_mut();

        // The root is an index node
        // check if the key is in the tree
        let (exist, child_index) = root.get_index_of(key);

        // recursively remove the subtree root
        let underflow = {
            let child = root.get_child_mut(child_index).unwrap();
            child.remove_recur(&key);
            child.root.borrow().is_underflow()
        };

        if underflow {
            // If the child node is underflow, merge or rebalance it with its sibling node
            // It is guaranteed that the sibling node is not empty
            let sibling_index = root.get_sibiling_index(child_index);
            let sibiling_is_left = sibling_index < child_index;
            let sibling = root.get_child(sibling_index).unwrap();
            if sibling.root.borrow().is_minimum() {
                // if the sibling node is minimum, merge it with the child node
                root.merge_children(child_index, sibiling_is_left);
            } else {
                // if the sibling node is not minimum, rebalance it with the child node
                root.rebalance_children(child_index, sibiling_is_left);
            }
        } else if exist {
            // Find the successor and replace the key
            let child = root.get_child(child_index).unwrap();
            let successor = BPTree::minimum(&child.root);
            root.set_key(child_index - 1, successor);
        }
    }

    fn minimum(node: &BPNodePtr<FANOUT, K, V>) -> K {
        let node = node.borrow();
        if let BPNode::Index(inode) = node.deref() {
            return BPTree::minimum(&inode.get_child(0).unwrap().root);
        }
        *node.as_leaf().get_key(0).unwrap()
    }
}
