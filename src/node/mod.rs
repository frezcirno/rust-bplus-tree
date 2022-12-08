mod bp_index_node;
mod bp_leaf_node;
use std::fmt::Debug;

pub use bp_index_node::BPIndexNode;
pub use bp_leaf_node::BPLeafNode;

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

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

    pub fn search_key(&self, key: &K) -> Result<usize, usize> {
        match self {
            BPNode::Leaf(leaf) => leaf.search_key(key),
            BPNode::Index(index) => index.search_key(key),
        }
    }
}
