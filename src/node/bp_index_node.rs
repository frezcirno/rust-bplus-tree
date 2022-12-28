use super::{BPNode, BPNodePtr, BPNodeWeak};
use std::cell::RefCell;
use std::fmt::Debug;
use std::ops::DerefMut;
use std::rc::Rc;

pub struct BPIndexNode<const FANOUT: usize, K: Copy + Ord + Debug, V: Clone + Debug> {
    keys: Vec<K>,
    children: Vec<BPNodePtr<FANOUT, K, V>>,
    parent: Option<BPNodeWeak<FANOUT, K, V>>,
    pub prev: Option<BPNodeWeak<FANOUT, K, V>>,
    pub next: Option<BPNodePtr<FANOUT, K, V>>,
}

impl<const FANOUT: usize, K: Copy + Ord + Debug, V: Clone + Debug> Debug
    for BPIndexNode<FANOUT, K, V>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BPIndexNode {{ keys: {:?}, {:?} children}}",
            self.keys,
            self.children.len(),
        )
    }
}

impl<const FANOUT: usize, K: Copy + Ord + Debug, V: Clone + Debug> BPIndexNode<FANOUT, K, V> {
    pub fn new() -> Self {
        BPIndexNode {
            keys: Vec::new(),
            children: Vec::new(),
            parent: None,
            prev: None,
            next: None,
        }
    }

    pub fn new_with(
        keys: Vec<K>,
        children: Vec<BPNodePtr<FANOUT, K, V>>,
        parent: Option<BPNodeWeak<FANOUT, K, V>>,
        prev: Option<BPNodeWeak<FANOUT, K, V>>,
        next: Option<BPNodePtr<FANOUT, K, V>>,
    ) -> Self {
        BPIndexNode {
            keys,
            children,
            parent,
            prev,
            next,
        }
    }

    pub fn is_full(&self) -> bool {
        self.keys.len() == FANOUT
    }

    pub fn is_maxinum(&self) -> bool {
        self.children.len() == FANOUT - 1
    }

    pub fn is_minimum(&self) -> bool {
        self.children.len() == (FANOUT + 1) / 2
    }

    pub fn is_underflow(&self) -> bool {
        self.children.len() < (FANOUT + 1) / 2
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    pub fn get_key(&self, index: usize) -> Option<&K> {
        self.keys.get(index)
    }

    pub fn get_child(&self, index: usize) -> Option<&BPNodePtr<FANOUT, K, V>> {
        self.children.get(index)
    }

    pub fn get_child_clone(&self, index: usize) -> Option<BPNodePtr<FANOUT, K, V>> {
        let child = self.children.get(index)?;
        Some(child.clone())
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

    pub fn push_key(&mut self, key: K) {
        self.keys.push(key);
    }

    pub fn push_child(&mut self, child: BPNodePtr<FANOUT, K, V>) {
        self.children.push(child);
    }

    pub(crate) fn insert_key_at(&mut self, index: usize, key: K) {
        self.keys.insert(index, key);
    }

    pub fn remove_key(&mut self, index: usize) -> K {
        self.keys.remove(index)
    }

    pub fn set_key(&mut self, index: usize, key: K) {
        self.keys[index] = key;
    }

    pub(crate) fn insert_child_at(&mut self, index: usize, child: BPNodePtr<FANOUT, K, V>) {
        self.children.insert(index, child);
    }

    pub fn remove_child(&mut self, index: usize) -> BPNodePtr<FANOUT, K, V> {
        self.children.remove(index)
    }

    pub fn search_key(&self, key: &K) -> Result<usize, usize> {
        self.keys.binary_search(key)
    }

    pub fn get_index_of(&self, key: &K) -> (bool, usize) {
        match self.keys.binary_search(key) {
            Ok(index) => (true, index + 1),
            Err(index) => (false, index),
        }
    }

    pub fn split_node(node: &BPNodePtr<FANOUT, K, V>,inode: &mut BPIndexNode<FANOUT, K, V>) -> (K, BPNodePtr<FANOUT, K, V>) {
        let split_key = *inode.get_key(FANOUT / 2).unwrap();
        let new_index = BPIndexNode::new_with(
            inode.keys.split_off(FANOUT / 2 + 1),
            inode.children.split_off(FANOUT / 2 + 1),
            inode.parent.clone(),
            Some(Rc::<RefCell<BPNode<FANOUT, K, V>>>::downgrade(node)),
            inode.next.clone()
        );
        inode.keys.pop();
        let new_index_ptr = BPNode::new_index_ptr_from(new_index);
        inode.next = Some(new_index_ptr.clone());
        (split_key, new_index_ptr)
    }

    pub fn merge_children(&mut self, to_remove: usize, merge_into_left: bool) {
        // pop the key between the two children
        let key_index = if merge_into_left {
            to_remove - 1
        } else {
            to_remove
        };
        let key = self.keys.remove(key_index);

        // pop the child
        let child = self.children.remove(to_remove);

        let target_index = if merge_into_left {
            to_remove - 1
        } else {
            to_remove
        };
        let target = self.get_child(target_index).unwrap();

        match target.borrow_mut().deref_mut() {
            BPNode::Leaf(leaf) => {
                // strip the child node, and merge it into the target node
                let mut child = child.borrow_mut();
                leaf.merge(child.as_leaf_mut(), merge_into_left);
            }
            BPNode::Index(index) => {
                let mut child = child.borrow_mut();
                let child0 = child.as_index_mut().remove_child(0);
                if merge_into_left {
                    index.keys.push(key);
                    index.children.push(child0);
                    index.next = child.as_index_mut().next.take();
                } else {
                    index.keys.insert(0, key);
                    index.children.insert(0, child0);
                    index.prev = child.as_index_mut().prev.take();
                }
            }
        }
    }

    pub fn rebalance_children(&mut self, target_index: usize, rebalance_from_left: bool) {
        // pop the key between the two children
        let key_index = if rebalance_from_left {
            target_index - 1
        } else {
            target_index
        };
        let key = self.keys.remove(key_index);

        //
        let from_index = if rebalance_from_left {
            target_index - 1
        } else {
            target_index + 1
        };
        let from = self.get_child_clone(from_index).unwrap();
        let mut from = from.borrow_mut();

        let target = self.get_child_clone(target_index).unwrap();

        match target.borrow_mut().deref_mut() {
            BPNode::Leaf(leaf) => {
                leaf.steal(from.as_leaf_mut(), rebalance_from_left);
            }
            BPNode::Index(index) => {
                assert!(index.keys.len() == 0);
                assert!(index.children.len() == 1);
                let from = from.as_index_mut();

                if rebalance_from_left {
                    index.keys.insert(0, key);
                    index.children.insert(0, from.remove_child(0));
                    let from_key = from.remove_key(0);
                    self.insert_key_at(key_index, from_key);
                } else {
                    index.keys.push(key);
                    index.children.push(from.remove_child(0));
                    self.insert_key_at(key_index, from.remove_key(0));
                }
            }
        };
    }

    pub fn get_sibiling_index(&self, index: usize) -> usize {
        let sibiling_is_left = index > 0;
        if sibiling_is_left {
            index - 1
        } else {
            index + 1
        }
    }
}
