use std::fmt::Debug;
use std::{cell::RefCell, rc::Rc};

use super::{BPNode, BPNodePtr, BPNodeWeak};

pub struct BPLeafNode<const FANOUT: usize, K: Copy + Ord + Debug, V: Clone + Debug> {
    keys: Vec<K>,
    values: Vec<V>,
    parent: Option<BPNodeWeak<FANOUT, K, V>>,
    prev: Option<BPNodeWeak<FANOUT, K, V>>,
    next: Option<BPNodePtr<FANOUT, K, V>>,
}

impl<const FANOUT: usize, K: Copy + Ord + Debug, V: Clone + Debug> Debug
    for BPLeafNode<FANOUT, K, V>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BPLeafNode")
            .field("keys", &self.keys)
            .field("values", &self.values)
            .finish()
    }
}

impl<const FANOUT: usize, K: Copy + Ord + Debug, V: Clone + Debug> BPLeafNode<FANOUT, K, V> {
    pub fn new() -> Self {
        BPLeafNode {
            keys: Vec::new(),
            values: Vec::new(),
            parent: None,
            prev: None,
            next: None,
        }
    }

    pub fn new_with(
        keys: Vec<K>,
        values: Vec<V>,
        parent: Option<BPNodeWeak<FANOUT, K, V>>,
        prev: Option<BPNodeWeak<FANOUT, K, V>>,
        next: Option<BPNodePtr<FANOUT, K, V>>,
    ) -> Self {
        BPLeafNode {
            keys,
            values,
            parent,
            prev,
            next,
        }
    }

    pub fn is_full(&self) -> bool {
        self.keys.len() == FANOUT
    }

    pub fn is_minimum(&self) -> bool {
        self.keys.len() == FANOUT / 2
    }

    pub fn is_underflow(&self) -> bool {
        self.keys.len() < FANOUT / 2
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    pub fn get_key(&self, index: usize) -> Option<&K> {
        self.keys.get(index)
    }

    pub fn get_value(&self, index: usize) -> Option<&V> {
        self.values.get(index)
    }

    pub fn delete(&mut self, key: &K) -> bool {
        let index = self.keys.binary_search(key).ok();
        if let Some(index) = index {
            self.keys.remove(index);
            self.values.remove(index);
            true
        } else {
            false
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<(K, V)> {
        if index < self.keys.len() {
            let key = self.keys.remove(index);
            let value = self.values.remove(index);
            Some((key, value))
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> bool {
        let index = match self.keys.binary_search(&key) {
            Ok(_) => return false,
            Err(index) => index,
        };
        self.keys.insert(index, key);
        self.values.insert(index, value);
        true
    }

    pub fn split_leaf_node(
        node: &BPNodePtr<FANOUT, K, V>,
        leaf: &mut BPLeafNode<FANOUT, K, V>,
    ) -> (K, BPNodePtr<FANOUT, K, V>) {
        let split_key = *leaf.get_key(FANOUT / 2).unwrap();
        let new_leaf = BPLeafNode::new_with(
            leaf.keys.split_off(FANOUT / 2),
            leaf.values.split_off(FANOUT / 2),
            leaf.parent.clone(),
            Some(Rc::<RefCell<BPNode<FANOUT, K, V>>>::downgrade(node)),
            leaf.next.clone(),
        );
        let new_leaf_ptr = BPNode::new_leaf_ptr_from(new_leaf);
        leaf.next = Some(new_leaf_ptr.clone());
        (split_key, new_leaf_ptr)
    }

    pub fn search_key(&self, key: &K) -> Result<usize, usize> {
        self.keys.binary_search(key)
    }

    pub fn push_key_value(&mut self, key: K, value: V) {
        self.keys.push(key);
        self.values.push(value);
    }

    pub fn insert_key_value(&mut self, index: usize, key: K, value: V) {
        self.keys.insert(index, key);
        self.values.insert(index, value);
    }

    pub fn merge(&mut self, other: &mut BPLeafNode<FANOUT, K, V>, other_is_next: bool) {
        if other_is_next {
            self.keys.append(&mut other.keys);
            self.values.append(&mut other.values);
            self.next = other.next.take();
            if let Some(next) = self.next.as_ref() {
                next.borrow_mut().as_leaf_mut().prev = other.prev.take();
            }
        } else {
            // merge to front
            self.keys.insert(0, other.keys.pop().unwrap());
            self.values.insert(0, other.values.pop().unwrap());
            self.prev = other.prev.take();
            if let Some(prev) = self.prev.as_ref() {
                prev.upgrade().unwrap().borrow_mut().as_leaf_mut().next = other.next.take();
            }
        }
        other.parent.take();
    }

    pub fn steal(&mut self, other: &mut BPLeafNode<FANOUT, K, V>, other_is_next: bool) {
        if other_is_next {
            self.keys.push(other.keys.remove(0));
            self.values.push(other.values.remove(0));
        } else {
            // steal from front
            self.keys.insert(0, other.keys.pop().unwrap());
            self.values.insert(0, other.values.pop().unwrap());
        }
    }
}
