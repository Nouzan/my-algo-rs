use super::super::linked_binary_tree::{cursor::CursorMut, LinkedBinaryTree};
use super::super::{BinTree, BinTreeCursor, BinTreeCursorMut, BinTreeMut};
use std::mem;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct LeftNode<T> {
    elem: T,
    npl: usize,
}

/// (大顶)左式堆.
pub struct LeftHeap<T> {
    tree: LinkedBinaryTree<LeftNode<T>>,
}

impl<T> Default for LeftHeap<T> {
    fn default() -> Self {
        Self {
            tree: LinkedBinaryTree::default(),
        }
    }
}

impl<T: PartialOrd + 'static> LeftHeap<T> {
    fn merge_inner<'a>(
        lhs: &mut CursorMut<'a, LeftNode<T>>,
        mut rhs: LinkedBinaryTree<LeftNode<T>>,
    ) {
        if !rhs.is_empty() {
            if lhs.right().is_some() {
                if lhs.right().unwrap() < rhs.cursor().as_ref().unwrap() {
                    let subtree = lhs.take_right().unwrap();
                    Self::merge_inner(&mut rhs.cursor_mut(), subtree);
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

    fn merge(&mut self, other: Self) {
        if !self.tree.is_empty() {
            Self::merge_inner(&mut self.tree.cursor_mut(), other.tree);
        } else if !other.tree.is_empty() {
            self.tree = other.tree;
        }
    }
}
