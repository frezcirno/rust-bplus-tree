use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use crate::node::{BPIndexNode, BPLeafNode};
use crate::node::{BPNode, BPNodePtr};

pub struct BPTree<const FANOUT: usize, K: Copy + Ord + Debug, V: Clone + Debug> {
    root: BPNodePtr<FANOUT, K, V>,
}

impl<const FANOUT: usize, K: Copy + Ord + Debug, V: Clone + Debug> BPTree<FANOUT, K, V> {
    pub fn new() -> Self {
        BPTree {
            root: Rc::new(RefCell::new(BPNode::Leaf(BPLeafNode::new()))),
        }
    }

    pub fn root_replace(&mut self, new_root: BPNodePtr<FANOUT, K, V>) -> BPNodePtr<FANOUT, K, V> {
        std::mem::replace(&mut self.root, new_root)
    }

    pub fn locate_leaf_for_insert(&self, key: &K) -> BPNodePtr<FANOUT, K, V> {
        let mut node = self.root.clone();
        while let BPNode::Index(inode) = node.clone().borrow().deref() {
            match inode.search_key(&key) {
                Ok(idx) => {
                    node = inode.get_child(idx + 1).unwrap().clone();
                }
                Err(pos) => {
                    node = inode.get_child(pos).unwrap().clone();
                }
            }
        }
        node
    }

    pub fn locate_leaf_for_delete(&self, key: &K) -> Option<BPNodePtr<FANOUT, K, V>> {
        let mut node = self.root.clone();
        while let BPNode::Index(inode) = node.clone().borrow().deref() {
            match inode.search_key(&key) {
                Ok(idx) => {
                    node = inode.get_child(idx + 1).unwrap().clone();
                }
                Err(_) => {
                    return None;
                }
            }
        }
        Some(node)
    }

    pub fn insert(&mut self, key: K, value: V) {
        let leafnode = self.locate_leaf_for_insert(&key);
        if leafnode.borrow_mut().as_leaf_mut().insert(key, value) {
            if leafnode.borrow().is_full() {
                self.adjust_node(&leafnode);
            }
        }
    }

    pub fn split_node(node: &BPNodePtr<FANOUT, K, V>) -> (K, BPNodePtr<FANOUT, K, V>) {
        match node.borrow_mut().deref_mut() {
            BPNode::Leaf(leaf) => BPLeafNode::split_leaf_node(node, leaf),
            BPNode::Index(index) => BPIndexNode::split_index_node(index),
        }
    }

    pub fn adjust_node(&mut self, node: &BPNodePtr<FANOUT, K, V>) {
        let (split_key, right) = BPTree::split_node(node);

        if let Some(parent) = node.borrow().get_parent() {
            let parent = parent.upgrade().unwrap();
            if {
                let mut parent = parent.borrow_mut();
                let parent = parent.as_index_mut();
                let index = parent.search_key(&split_key).unwrap_err();
                parent.insert_key_at(index, split_key);
                parent.insert_child_at(index + 1, right.clone());
                parent.is_full()
            } {
                self.adjust_node(&parent);
            }

            return;
        }

        // We are splitting the root node
        // create a new index node and make it the root
        let old_root = self.root_replace(BPNode::new_index_ptr());

        // insert the old root and the new node as children of the new root
        old_root
            .borrow_mut()
            .set_parent(Some(Rc::downgrade(&self.root)));
        right
            .borrow_mut()
            .set_parent(Some(Rc::downgrade(&self.root)));

        let mut root = self.root.borrow_mut();
        let iroot = root.as_index_mut();
        iroot.push_key(split_key);
        iroot.push_child(old_root);
        iroot.push_child(right.clone());
    }

    pub fn locate_index_node(node: &BPNodePtr<FANOUT, K, V>) -> Option<BPNodePtr<FANOUT, K, V>> {
        let mut node = node.clone();
        while node.borrow().get_self_index()? == 0 {
            node = {
                let node = node.borrow();
                let parent = node.get_parent().unwrap();
                parent.upgrade().unwrap()
            };
        }
        Some(node)
    }

    pub fn delete(&mut self, key: &K) {
        if let Some(leafnode) = self.locate_leaf_for_delete(key) {
            let is_first = {
                let mut leafnode = leafnode.borrow_mut();
                let leaf = leafnode.as_leaf_mut();
                let is_first = leaf.get_key(0) == Some(key);
                leaf.delete(key);
                is_first
            };

            // TODO
            // if leaf.is_empty() {
            //     self.adjust_underflow(&leafnode);
            // }

            // if is_first {
            //     if let Some(pnode) = BPTree::locate_index_node(leafnode) {
            //         if let Some(ppnode) = pnode.borrow().get_parent() {
            //             if let Some(index) = pnode.borrow().get_self_index() {
            //                 if index > 0 {
            //                     let ppnode = ppnode.upgrade().unwrap();

            //                 }
            //             }
            //         }
            //     }
            // }
        }
    }

    pub fn print(&self) {
        let node = &self.root;
        let mut queue = VecDeque::new();
        queue.push_back(node.clone());
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            match node.borrow().deref() {
                BPNode::Leaf(leaf) => {
                    println!("Leaf: {:?}", leaf);
                }
                BPNode::Index(index) => {
                    println!("Index: {:?}", index);
                    for child in index.get_children() {
                        queue.push_back(child.clone());
                    }
                }
            };
        }
    }
}
