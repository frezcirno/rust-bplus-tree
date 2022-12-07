use crate::bp_tree::BPTree;

use super::{BPNode, BPNodePtr, BPNodeWeak};
use std::fmt::Debug;
use std::ops::DerefMut;
use std::rc::Rc;

pub struct BPIndexNode<const FANOUT: usize, K: Copy + Ord + Debug, V: Clone + Debug> {
    keys: Vec<K>,
    children: Vec<BPTree<FANOUT, K, V>>,
    parent: Option<BPNodeWeak<FANOUT, K, V>>,
}

impl<const FANOUT: usize, K: Copy + Ord + Debug, V: Clone + Debug> Debug
    for BPIndexNode<FANOUT, K, V>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BPIndexNode {{ keys: {:?} }}", self.keys)
    }
}

impl<const FANOUT: usize, K: Copy + Ord + Debug, V: Clone + Debug> BPIndexNode<FANOUT, K, V> {
    pub fn new() -> Self {
        BPIndexNode {
            keys: Vec::new(),
            children: Vec::new(),
            parent: None,
        }
    }

    pub fn new_with(
        keys: Vec<K>,
        children: Vec<BPTree<FANOUT, K, V>>,
        parent: Option<BPNodeWeak<FANOUT, K, V>>,
    ) -> Self {
        BPIndexNode {
            keys,
            children,
            parent,
        }
    }

    pub fn is_full(&self) -> bool {
        self.keys.len() == FANOUT
    }

    pub fn is_minimum(&self) -> bool {
        self.children.len() == FANOUT / 2
    }

    pub fn is_underflow(&self) -> bool {
        self.children.len() < FANOUT / 2
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    pub fn size(&self) -> usize {
        self.keys.len()
    }

    pub fn get_key(&self, index: usize) -> Option<&K> {
        self.keys.get(index)
    }

    pub fn get_child(&self, index: usize) -> Option<&BPTree<FANOUT, K, V>> {
        self.children.get(index)
    }

    pub fn get_child_mut(&mut self, index: usize) -> Option<&mut BPTree<FANOUT, K, V>> {
        self.children.get_mut(index)
    }

    pub fn get_children(&self) -> &Vec<BPTree<FANOUT, K, V>> {
        &self.children
    }

    pub fn get_parent(&self) -> Option<&BPNodeWeak<FANOUT, K, V>> {
        self.parent.as_ref()
    }

    pub fn set_parent(&mut self, parent: Option<BPNodeWeak<FANOUT, K, V>>) {
        self.parent = parent;
    }

    pub fn push_key(&mut self, key: K) {
        self.keys.push(key);
    }

    pub fn push_child(&mut self, child: BPTree<FANOUT, K, V>) {
        self.children.push(child);
    }

    pub fn insert_key_at(&mut self, index: usize, key: K) {
        self.keys.insert(index, key);
    }

    pub fn insert_key(&mut self, key: K) {
        let index = match self.keys.binary_search(&key) {
            Ok(index) => index,
            Err(index) => index,
        };
        self.keys.insert(index, key);
    }

    pub fn remove_key(&mut self, index: usize) -> K {
        self.keys.remove(index)
    }

    pub fn set_key(&mut self, index: usize, key: K) {
        self.keys[index] = key;
    }

    pub fn insert_child_at(&mut self, index: usize, child: BPTree<FANOUT, K, V>) {
        self.children.insert(index, child);
    }

    pub fn insert_child(&mut self, index: usize, child: BPTree<FANOUT, K, V>) {
        self.children.insert(index, child);
    }

    pub fn push_key_child(&mut self, key: K, child: BPTree<FANOUT, K, V>) {
        self.keys.push(key);
        self.children.push(child);
    }

    pub fn remove_child(&mut self, index: usize) -> BPTree<FANOUT, K, V> {
        self.children.remove(index)
    }

    pub fn remove_key_lchild(&mut self, index: usize) -> Option<(K, BPTree<FANOUT, K, V>)> {
        Some((self.keys.remove(index), self.children.remove(index)))
    }

    pub fn remove_key_rchild(&mut self, index: usize) -> Option<(K, BPTree<FANOUT, K, V>)> {
        Some((self.keys.remove(index), self.children.remove(index + 1)))
    }

    pub fn search_key(&self, key: &K) -> Result<usize, usize> {
        self.keys.binary_search(key)
    }

    pub fn get_index_of(&self, key: &K) -> usize {
        match self.keys.binary_search(key) {
            Ok(index) => index + 1,
            Err(index) => index,
        }
    }

    pub fn search_child(&self, node: &BPTree<FANOUT, K, V>) -> Option<usize> {
        for (i, child) in self.children.iter().enumerate() {
            if Rc::ptr_eq(&child.root, &node.root) {
                return Some(i);
            }
        }
        None
    }

    pub fn split_node(inode: &mut BPIndexNode<FANOUT, K, V>) -> (K, BPNodePtr<FANOUT, K, V>) {
        let split_key = *inode.get_key(FANOUT / 2).unwrap();
        let new_index = BPIndexNode::new_with(
            inode.keys.split_off(FANOUT / 2 + 1),
            inode.children.split_off(FANOUT / 2 + 1),
            inode.parent.clone(),
        );
        inode.keys.pop();
        (split_key, BPNode::new_index_ptr_from(new_index))
    }

    pub fn merge_children(&mut self, child_index: usize, merge_into_left: bool) {
        // pop the key between the two children
        let key_index = if merge_into_left {
            child_index - 1
        } else {
            child_index
        };
        let key = self.keys.remove(key_index);

        // pop the child
        let child = self.children.remove(child_index);

        let target_index = if merge_into_left {
            child_index - 1
        } else {
            child_index
        };
        let target = self.get_child(target_index).unwrap();

        match target.root.borrow_mut().deref_mut() {
            BPNode::Leaf(leaf) => {
                // strip the child node, and merge it into the target node
                let mut child = child.root.borrow_mut();
                leaf.merge(child.as_leaf_mut(), merge_into_left);
            }
            BPNode::Index(index) => {
                assert!(index.keys.len() == 0);
                assert!(index.children.len() == 1);

                if merge_into_left {
                    index.keys.insert(0, key);
                    index.children.insert(0, child);
                } else {
                    index.keys.push(key);
                    index.children.push(child);
                }
            }
        }
    }

    pub fn rebalance_child(&mut self, left: usize) {}

    pub fn get_sibiling_index(&self, index: usize) -> usize {
        let sibiling_is_left = index > 0;
        if sibiling_is_left {
            index - 1
        } else {
            index + 1
        }
    }
}
