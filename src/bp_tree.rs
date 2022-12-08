use std::collections::VecDeque;
use std::fmt::Debug;
use std::ops::Deref;

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
                        queue.push_back(child.clone());
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
            root: BPNode::new_leaf_ptr(),
        }
    }

    pub fn new_from(root: BPNodePtr<FANOUT, K, V>) -> Self {
        BPTree { root }
    }

    fn root_replace(&mut self, new_root: BPNodePtr<FANOUT, K, V>) -> BPNodePtr<FANOUT, K, V> {
        std::mem::replace(&mut self.root, new_root)
    }

    pub fn search(&self, key: &K) -> Option<V> {
        BPNode::search_recur(&self.root, key)
    }

    pub fn insert(&mut self, key: K, value: V) {
        BPNode::insert_recur(&mut self.root, key, value);
        if self.root.borrow().deref().is_full() {
            let old_root = self.root_replace(BPNode::new_index_ptr());
            let (split_key, right) = BPNode::split_node(&old_root);
            let mut root = self.root.borrow_mut();
            let root = root.as_index_mut();
            root.push_key(split_key);
            root.push_child(old_root);
            root.push_child(right);
        }
    }

    pub fn remove(&mut self, key: &K) {
        BPNode::remove_recur(&mut self.root, key);
        if self.root.borrow().is_empty() {
            let child = self.root.borrow_mut().as_index_mut().remove_child(0);
            self.root_replace(child);
        }
    }
}
