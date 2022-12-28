mod bp_index_node;
mod bp_leaf_node;
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

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

    pub fn is_maxinum(&self) -> bool {
        match self {
            BPNode::Leaf(leaf) => leaf.is_maxinum(),
            BPNode::Index(index) => index.is_maxinum(),
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

    pub(crate) fn insert_recur(root: &BPNodePtr<FANOUT, K, V>, key: K, value: V) {
        let mut root = root.borrow_mut();

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
                    let child_num = iroot.get_children().len();
                    let mut change_key=iroot.get_key(0).unwrap().clone();
                    let mut old_key = iroot.get_key(0).unwrap().clone();
                    let child = iroot.get_child_mut(index).unwrap();
                    Self::insert_recur(child, key, value);
                    if child.borrow_mut().is_full() {
                        let mut is_changed = false;
                        if index < child_num-1 {
                            {
                                let mut cb = child.borrow_mut();
                                match cb.deref(){
                                    BPNode::Leaf(lroot) => {
                                        if let Some(nextnode) = lroot.next.clone() {
                                            if ! nextnode.borrow_mut().is_maxinum(){
                                                let removal = cb.as_leaf_mut().remove(FANOUT-1);
                                                is_changed = true;
                                                if let Some(tmp_removel) = removal{
                                                    change_key = tmp_removel.0;
                                                    let val = tmp_removel.1;
                                                    old_key = *nextnode.borrow_mut().as_leaf_mut().get_key(0).unwrap();
                                                    nextnode.borrow_mut().as_leaf_mut().insert(change_key.clone(), val);
                                                }
                                            }
                                        }
                                    }
                                    BPNode::Index(iroot) => { 
                                        if let Some(nextnode) = iroot.next.clone() {
                                            if ! nextnode.borrow_mut().is_maxinum(){
                                                is_changed = true;
                                                change_key = child.borrow_mut().as_index_mut().remove_key(FANOUT-1);
                                                old_key = *nextnode.borrow_mut().as_index_mut().get_key(0).unwrap();
                                                nextnode.borrow_mut().as_index_mut().insert_key_at(0, change_key.clone());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if is_changed{
                            let (is_find, ind) = iroot.get_index_of(&old_key);
                            if is_find{
                                iroot.set_key(ind-1, change_key);
                            }
                        }
                        else {
                            let (split_key, right) = Self::split_node(&child);
                            iroot.insert_key_at(index, split_key);
                            iroot.insert_child_at(index + 1, right);
                        }
                    }
                }
            }
        }
    }

    pub(crate) fn remove_recur(root: &BPNodePtr<FANOUT, K, V>, key: &K) {
        let mut root = root.borrow_mut();

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

        // The root is an index node
        let root = root.as_index_mut();

        // check if the key is in the tree root
        let (exist, child_index) = root.get_index_of(key);

        let underflow = {
            // recursively remove the subtree root
            let child = root.get_child_mut(child_index).unwrap();
            Self::remove_recur(child, &key);
            child.borrow().is_underflow()
        };

        if underflow {
            // If the child node is underflow, merge or rebalance it with its sibling node
            // It is guaranteed that the sibling node is not empty
            let sibling_index = root.get_sibiling_index(child_index);
            let sibiling_is_left = sibling_index < child_index;
            let sibling = root.get_child(sibling_index).unwrap();
            if sibling.borrow().is_minimum() {
                // if the sibling node is minimum, merge it with the child node
                root.merge_children(child_index, sibiling_is_left);
            } else {
                // if the sibling node is not minimum, rebalance it with the child node
                root.rebalance_children(child_index, sibiling_is_left);
            }
        } else if exist {
            // Find the successor and replace the key
            let child = root.get_child(child_index).unwrap();
            let successor = BPNode::minimum(&child);
            root.set_key(child_index - 1, successor);
        }
    }

    pub(crate) fn split_node(node: &BPNodePtr<FANOUT, K, V>) -> (K, BPNodePtr<FANOUT, K, V>) {
        match node.borrow_mut().deref_mut() {
            BPNode::Leaf(leaf) => BPLeafNode::split_leaf_node(node, leaf),
            BPNode::Index(index) => BPIndexNode::split_node(node, index),
        }
    }

    pub fn minimum(node: &BPNodePtr<FANOUT, K, V>) -> K {
        let node = node.borrow();
        if let BPNode::Index(inode) = node.deref() {
            return Self::minimum(&inode.get_child(0).unwrap());
        }
        *node.as_leaf().get_key(0).unwrap()
    }

    pub(crate) fn search_recur(root: &BPNodePtr<FANOUT, K, V>, key: &K) -> Option<V> {
        let root = root.borrow();
        match root.deref() {
            BPNode::Leaf(leaf) => leaf
                .search_key(&key)
                .ok()
                .map(|index| leaf.get_value(index).unwrap())
                .cloned(),
            BPNode::Index(index) => {
                let (_, idx) = index.get_index_of(&key);
                let child = index.get_child(idx).unwrap();
                Self::search_recur(child, key)
            }
        }
    }
}
