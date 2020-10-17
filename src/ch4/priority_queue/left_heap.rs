use super::super::linked_binary_tree::{cursor::CursorMut, LinkedBinaryTree};
use super::super::{BinTree, BinTreeCursor, BinTreeCursorMut, BinTreeMut};
use super::PriorityQueue;
use crate::vec::MyVec;
use std::mem::swap;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct LeftNode<T> {
    elem: T,
    npl: usize,
}

/// (大顶)左式堆.
pub struct LeftHeap<T> {
    len: usize,
    tree: LinkedBinaryTree<LeftNode<T>>,
}

impl<T> Default for LeftHeap<T> {
    fn default() -> Self {
        Self {
            len: 0,
            tree: LinkedBinaryTree::default(),
        }
    }
}

impl<T: PartialOrd + 'static> LeftHeap<T> {
    fn merge_inner<'a>(lhs: &mut CursorMut<'a, LeftNode<T>>, rhs: LinkedBinaryTree<LeftNode<T>>) {
        if !rhs.is_empty() {
            if lhs.right().is_some() {
                if lhs.right().unwrap() < rhs.cursor().as_ref().unwrap() {
                    let subtree = lhs.take_right().unwrap();
                    lhs.append_right(rhs);
                    lhs.move_right();
                    Self::merge_inner(lhs, subtree);
                } else {
                    lhs.move_right();
                    Self::merge_inner(lhs, rhs);
                }
            } else {
                lhs.append_right(rhs);
            }
            let (lc, rc) = (lhs.left(), lhs.right());
            let lnpl = lc.map_or(0, |node| node.npl);
            let rnpl = rc.map_or(0, |node| node.npl);
            lhs.as_mut().unwrap().npl = 1 + lnpl.min(rnpl);
            if lnpl < rnpl {
                let (left, right) = (lhs.take_left().unwrap(), lhs.take_right().unwrap());
                lhs.append_left(right);
                lhs.append_right(left);
            }
        }
    }
}

impl<T: PartialOrd + 'static> From<MyVec<T>> for LeftHeap<T> {
    fn from(mut vec: MyVec<T>) -> Self {
        let mut heap = Self::default();
        while let Some(elem) = vec.pop() {
            heap.insert(elem);
        }
        heap
    }
}

impl<T: PartialOrd + 'static> PriorityQueue<T> for LeftHeap<T> {
    fn insert(&mut self, elem: T) {
        let mut heap = Self::default();
        heap.tree
            .cursor_mut()
            .insert_as_root(LeftNode { elem, npl: 1 });
        heap.len = 1;
        self.merge(heap);
    }

    fn merge(&mut self, mut other: Self) {
        let len = self.len + other.len;
        if !self.tree.is_empty() {
            if let Some(rhs) = other.tree.cursor().as_ref() {
                let lhs = self.tree.cursor().into_ref().unwrap();
                if lhs < rhs {
                    swap(&mut self.tree, &mut other.tree);
                }
            }
            Self::merge_inner(&mut self.tree.cursor_mut(), other.tree);
        } else if !other.tree.is_empty() {
            self.tree = other.tree;
        }
        self.len = len;
    }

    fn len(&self) -> usize {
        self.len
    }

    fn get_max(&self) -> Option<&T> {
        self.tree.cursor().into_ref().map(|node| &node.elem)
    }

    fn delete_max(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let len = self.len - 1;
            let mut cursor = self.tree.cursor_mut();
            let (left, right) = (cursor.take_left().unwrap(), cursor.take_right().unwrap());
            let elem = cursor.into_inner().map(|node| node.elem);
            let mut lhs = Self { len, tree: left };
            let rhs = Self {
                len: 0,
                tree: right,
            };
            lhs.merge(rhs);
            *self = lhs;
            elem
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ch4::BinTreeCursorExt;
    use proptest::prelude::*;

    proptest! {
        // #[test]
        // fn test_basic(mut data: Vec<i64>) {
        //     let sorted = CompleteMaxHeap::sort(MyVec::from(data.clone()));
        //     data.sort();
        //     for (idx, &elem) in sorted.iter().enumerate() {
        //         prop_assert_eq!(data[idx], elem);
        //     }
        // }

        #[test]
        fn test_priority(data1: Vec<i64>, data2: Vec<i64>) {
            let mut heap = LeftHeap::from(MyVec::from(data1.clone()));
            for &elem in data2.iter() {
                heap.insert(elem);
                let max = heap.tree.cursor().in_order_iter().map(|node| node.elem).max();
                let len = heap.tree.cursor().in_order_iter().count();
                assert_eq!(heap.len(), len);
                prop_assert_eq!(heap.delete_max(), max);
            }
        }

        #[test]
        fn test_merge(data1: Vec<i64>, data2: Vec<i64>) {
            let mut heap1 = LeftHeap::from(MyVec::from(data1));
            let heap2 = LeftHeap::from(MyVec::from(data2));
            heap1.merge(heap2);
            while !heap1.is_empty() {
                let max = heap1.tree.cursor().in_order_iter().map(|node| node.elem).max();
                let len = heap1.tree.cursor().in_order_iter().count();
                assert_eq!(heap1.len(), len);
                assert_eq!(heap1.delete_max(), max);
            }
        }
    }
}
