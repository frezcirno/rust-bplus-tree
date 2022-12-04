use super::{BPNode, BPNodePtr, BPNodeWeak};
use std::fmt::Debug;
use std::rc::Rc;

pub struct BPIndexNode<const FANOUT: usize, K: Copy + Ord + Debug, V: Clone + Debug> {
    keys: Vec<K>,
    children: Vec<BPNodePtr<FANOUT, K, V>>,
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
        children: Vec<BPNodePtr<FANOUT, K, V>>,
        parent: Option<BPNodeWeak<FANOUT, K, V>>,
    ) -> Self {
        BPIndexNode {
            keys,
            children,
            parent,
        }
    }

    pub fn is_full(&self) -> bool {
        self.children.len() == FANOUT
    }

    pub fn is_half_full(&self) -> bool {
        self.keys.len() == FANOUT / 2
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

    pub fn get_child(&self, index: usize) -> Option<&BPNodePtr<FANOUT, K, V>> {
        self.children.get(index)
    }

    pub fn get_child_mut(&mut self, index: usize) -> Option<&mut BPNodePtr<FANOUT, K, V>> {
        self.children.get_mut(index)
    }

    pub fn get_children(&self) -> &Vec<BPNodePtr<FANOUT, K, V>> {
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

    pub fn push_child(&mut self, child: BPNodePtr<FANOUT, K, V>) {
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

    pub fn insert_child_at(&mut self, index: usize, child: BPNodePtr<FANOUT, K, V>) {
        self.children.insert(index, child);
    }

    pub fn insert_child(&mut self, index: usize, child: BPNodePtr<FANOUT, K, V>) {
        self.children.insert(index, child);
    }

    pub fn push_key_child(&mut self, key: K, child: BPNodePtr<FANOUT, K, V>) {
        self.keys.push(key);
        self.children.push(child);
    }

    pub fn remove_key_child(&mut self, index: usize) -> Option<(K, BPNodePtr<FANOUT, K, V>)> {
        Some((self.keys.remove(index), self.children.remove(index)))
    }

    pub fn search_key(&self, key: &K) -> Result<usize, usize> {
        self.keys.binary_search(key)
    }

    pub fn search_child(&self, node: &BPNodePtr<FANOUT, K, V>) -> Option<usize> {
        for (i, child) in self.children.iter().enumerate() {
            if Rc::ptr_eq(child, node) {
                return Some(i);
            }
        }
        None
    }

    pub fn split_index_node(inode: &mut BPIndexNode<FANOUT, K, V>) -> (K, BPNodePtr<FANOUT, K, V>) {
        let split_key = *inode.get_key(FANOUT / 2).unwrap();
        let new_index = BPIndexNode::new_with(
            inode.keys.split_off(FANOUT / 2),
            inode.children.split_off(FANOUT / 2),
            inode.parent.clone(),
        );
        (split_key, BPNode::new_index_ptr_from(new_index))
    }
}
