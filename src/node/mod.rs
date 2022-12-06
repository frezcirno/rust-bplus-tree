mod bp_index_node;
mod bp_leaf_node;
use std::fmt::Debug;

pub use bp_index_node::BPIndexNode;
pub use bp_leaf_node::BPLeafNode;

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::bp_tree::BPTree;

pub type BPNodePtr<const FANOUT: usize, K, V> = Rc<RefCell<BPNode<FANOUT, K, V>>>;
pub type BPNodeWeak<const FANOUT: usize, K, V> = Weak<RefCell<BPNode<FANOUT, K, V>>>;

#[derive(Debug)]
pub enum BPNode<const FANOUT: usize, K: Copy + Ord + Debug, V: Clone + Debug> {
    Index(BPIndexNode<FANOUT, K, V>),
    Leaf(BPLeafNode<FANOUT, K, V>),
}

impl<const FANOUT: usize, K: Copy + Ord + Debug, V: Clone + Debug> BPNode<FANOUT, K, V> {
    pub fn new_leaf() -> Self {
        BPNode::Leaf(BPLeafNode::new())
    }

    pub fn new_index() -> Self {
        BPNode::Index(BPIndexNode::new())
    }

    pub fn new_leaf_ptr() -> BPNodePtr<FANOUT, K, V> {
        Rc::new(RefCell::new(BPNode::new_leaf()))
    }

    pub fn new_index_ptr() -> BPNodePtr<FANOUT, K, V> {
        Rc::new(RefCell::new(BPNode::new_index()))
    }

    pub fn new_leaf_ptr_from(lnode: BPLeafNode<FANOUT, K, V>) -> BPNodePtr<FANOUT, K, V> {
        Rc::new(RefCell::new(BPNode::Leaf(lnode)))
    }

    pub fn new_index_ptr_from(inode: BPIndexNode<FANOUT, K, V>) -> BPNodePtr<FANOUT, K, V> {
        Rc::new(RefCell::new(BPNode::Index(inode)))
    }

    pub fn is_leaf(&self) -> bool {
        match self {
            BPNode::Leaf(_) => true,
            BPNode::Index(_) => false,
        }
    }

    pub fn is_index(&self) -> bool {
        match self {
            BPNode::Leaf(_) => false,
            BPNode::Index(_) => true,
        }
    }

    pub fn as_leaf(&self) -> &BPLeafNode<FANOUT, K, V> {
        match self {
            BPNode::Leaf(leaf) => leaf,
            BPNode::Index(_) => panic!("not a leaf node"),
        }
    }

    pub fn as_index(&self) -> &BPIndexNode<FANOUT, K, V> {
        match self {
            BPNode::Leaf(_) => panic!("not an index node"),
            BPNode::Index(index) => index,
        }
    }

    pub fn as_leaf_mut(&mut self) -> &mut BPLeafNode<FANOUT, K, V> {
        match self {
            BPNode::Leaf(leaf) => leaf,
            BPNode::Index(_) => panic!("not a leaf node"),
        }
    }

    pub fn as_index_mut(&mut self) -> &mut BPIndexNode<FANOUT, K, V> {
        match self {
            BPNode::Leaf(_) => panic!("not an index node"),
            BPNode::Index(index) => index,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            BPNode::Leaf(leaf) => leaf.is_empty(),
            BPNode::Index(index) => index.is_empty(),
        }
    }

    pub fn is_full(&self) -> bool {
        match self {
            BPNode::Leaf(leaf) => leaf.is_full(),
            BPNode::Index(index) => index.is_full(),
        }
    }

    pub fn is_minimum(&self) -> bool {
        match self {
            BPNode::Leaf(leaf) => leaf.is_minimum(),
            BPNode::Index(index) => index.is_minimum(),
        }
    }

    pub fn is_underflow(&self) -> bool {
        match self {
            BPNode::Leaf(leaf) => leaf.is_underflow(),
            BPNode::Index(index) => index.is_underflow(),
        }
    }

    pub fn get_parent(&self) -> Option<&BPNodeWeak<FANOUT, K, V>> {
        match self {
            BPNode::Leaf(leaf) => leaf.get_parent(),
            BPNode::Index(index) => index.get_parent(),
        }
    }

    pub fn set_parent(&mut self, parent: Option<BPNodeWeak<FANOUT, K, V>>) {
        match self {
            BPNode::Leaf(leaf) => leaf.set_parent(parent),
            BPNode::Index(index) => index.set_parent(parent),
        }
    }

    pub fn get_self_index(&self) -> Option<usize> {
        let parent = match self {
            BPNode::Leaf(leaf) => leaf.get_parent(),
            BPNode::Index(index) => index.get_parent(),
        }?
        .upgrade()
        .unwrap();
        for (i, child) in parent.borrow().as_index().get_children().iter().enumerate() {
            if Rc::ptr_eq(&child.root, &parent) {
                return Some(i);
            }
        }
        unreachable!();
    }

    pub fn is_root(&self) -> bool {
        match self {
            BPNode::Leaf(leaf) => leaf.is_root(),
            BPNode::Index(index) => index.is_root(),
        }
    }

    pub fn search_key(&self, key: &K) -> Result<usize, usize> {
        match self {
            BPNode::Leaf(leaf) => leaf.search_key(key),
            BPNode::Index(index) => index.search_key(key),
        }
    }

    pub fn get_child(&self, index: usize) -> Option<&BPTree<FANOUT, K, V>> {
        match self {
            BPNode::Leaf(_) => panic!("not an index node"),
            BPNode::Index(inode) => inode.get_child(index),
        }
    }

    pub fn get_child_mut(&mut self, index: usize) -> Option<&mut BPTree<FANOUT, K, V>> {
        match self {
            BPNode::Leaf(_) => panic!("not an index node"),
            BPNode::Index(inode) => inode.get_child_mut(index),
        }
    }

    pub fn remove_child(&mut self, index: usize) -> BPTree<FANOUT, K, V> {
        match self {
            BPNode::Leaf(_) => panic!("not an index node"),
            BPNode::Index(inode) => inode.remove_child(index),
        }
    }

    pub fn get_key(&self, index: usize) -> Option<&K> {
        match self {
            BPNode::Leaf(lnode) => lnode.get_key(index),
            BPNode::Index(inode) => inode.get_key(index),
        }
    }

    pub fn set_key(&mut self, index: usize, key: K) {
        match self {
            BPNode::Leaf(lnode) => lnode.set_key(index, key),
            BPNode::Index(inode) => inode.set_key(index, key),
        }
    }

    pub fn push_key_child(&mut self, key: K, child: BPTree<FANOUT, K, V>) {
        match self {
            BPNode::Leaf(_) => panic!("not an index node"),
            BPNode::Index(inode) => inode.push_key_child(key, child),
        }
    }

    pub fn push_key_value(&mut self, key: K, value: V) {
        match self {
            BPNode::Leaf(leaf) => leaf.push_key_value(key, value),
            BPNode::Index(_) => panic!("not a leaf node"),
        }
    }

    pub fn insert_key_value(&mut self, index: usize, key: K, value: V) {
        match self {
            BPNode::Leaf(leaf) => leaf.insert_key_value(index, key, value),
            BPNode::Index(_) => panic!("not a leaf node"),
        }
    }
}
