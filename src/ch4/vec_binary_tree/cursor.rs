use super::super::{BaseNode, BaseNodeMut, BinTree, BinTreeMut, BinTreeNode, BinTreeNodeMut};
use super::{
    iter::{in_order_index, InOrderIndexIter, InOrderIter},
    VecBinaryTree,
};

pub(super) const fn left_index(index: usize) -> usize {
    2 * index + 1
}

pub(super) const fn right_index(index: usize) -> usize {
    2 * index + 2
}

pub struct Cursor<'a, T> {
    pub(super) current: usize,
    pub(super) tree: &'a VecBinaryTree<T>,
}

impl<'a, T> Clone for Cursor<'a, T> {
    fn clone(&self) -> Self {
        Self {
            current: self.current,
            tree: self.tree,
        }
    }
}

impl<'a, T> BaseNode<'a> for Cursor<'a, T> {
    type Elem = T;

    fn as_ref(&self) -> Option<&Self::Elem> {
        self.tree.get(self.current)
    }

    fn left(&self) -> Option<&Self::Elem> {
        self.tree.get(left_index(self.current))
    }

    fn right(&self) -> Option<&Self::Elem> {
        self.tree.get(right_index(self.current))
    }

    fn move_left(&mut self) {
        if !self.is_empty_subtree() {
            let idx = left_index(self.current);
            self.current = idx;
        }
    }

    fn move_right(&mut self) {
        if !self.is_empty_subtree() {
            let idx = right_index(self.current);
            self.current = idx;
        }
    }

    fn into_ref(self) -> Option<&'a Self::Elem>
    where
        Self: Sized,
    {
        self.tree.get(self.current)
    }
}

impl<'a, T> BinTreeNode<'a, VecBinaryTree<T>> for Cursor<'a, T> {
    fn new(tree: &'a VecBinaryTree<T>) -> Self {
        Self { current: 0, tree }
    }
}

impl<'a, T> BinTreeNode<'a, CursorMut<'a, T>> for Cursor<'a, T> {
    fn new(tree: &'a CursorMut<'a, T>) -> Self {
        Self {
            current: tree.current,
            tree: tree.tree,
        }
    }
}

pub struct CursorMut<'a, T> {
    pub(super) current: usize,
    pub(super) tree: &'a mut VecBinaryTree<T>,
}

impl<'a, T> CursorMut<'a, T> {
    fn get_left_index_and_resize(&mut self) -> usize {
        let idx = left_index(self.current);
        if idx >= self.tree.inner.len() {
            self.tree.inner.resize_with(idx + 1, || None);
        }
        idx
    }

    fn get_right_index_and_resize(&mut self) -> usize {
        let idx = right_index(self.current);
        if idx >= self.tree.inner.len() {
            self.tree.inner.resize_with(idx + 1, || None);
        }
        idx
    }

    fn get_node_and_resize(&mut self, index: usize) -> &mut Option<T> {
        if index >= self.tree.inner.len() {
            self.tree.inner.resize_with(index + 1, || None);
        }
        self.tree.inner.get_mut(index).unwrap()
    }

    pub(super) fn in_order_index_iter(&self) -> InOrderIndexIter {
        InOrderIndexIter::new(self.current, self.tree.inner.len())
    }

    pub fn in_order_iter(&self) -> InOrderIter<T> {
        InOrderIter::new(self.current, self.tree)
    }
}

impl<'a, T> BaseNode<'a> for CursorMut<'a, T> {
    type Elem = T;

    fn as_ref(&self) -> Option<&Self::Elem> {
        self.tree.get(self.current)
    }

    fn left(&self) -> Option<&Self::Elem> {
        self.tree.get(left_index(self.current))
    }

    fn right(&self) -> Option<&Self::Elem> {
        self.tree.get(right_index(self.current))
    }

    fn move_left(&mut self) {
        if !self.is_empty_subtree() {
            let idx = left_index(self.current);
            self.current = idx;
        }
    }

    fn move_right(&mut self) {
        if !self.is_empty_subtree() {
            let idx = right_index(self.current);
            self.current = idx;
        }
    }

    fn into_ref(self) -> Option<&'a Self::Elem>
    where
        Self: Sized,
    {
        self.tree.get(self.current)
    }
}

impl<'a, T> BaseNodeMut<'a> for CursorMut<'a, T> {
    fn as_mut(&mut self) -> Option<&mut Self::Elem> {
        self.tree.get_mut(self.current)
    }

    fn left_mut(&mut self) -> Option<&mut Self::Elem> {
        let idx = left_index(self.current);
        self.tree.get_mut(idx)
    }

    fn right_mut(&mut self) -> Option<&mut Self::Elem> {
        let idx = right_index(self.current);
        self.tree.get_mut(idx)
    }

    fn insert_as_root(&mut self, elem: Self::Elem) -> Option<Self::Elem> {
        if self.is_empty_subtree() {
            if self.current >= self.tree.inner.len() {
                self.tree.inner.resize_with(self.current + 1, || None);
            }
            let root = self.tree.inner.get_mut(self.current).unwrap();
            *root = Some(elem);
            None
        } else {
            Some(elem)
        }
    }

    fn insert_as_left(&mut self, elem: Self::Elem) -> Option<Self::Elem> {
        if self.left_mut().is_none() && !self.is_empty_subtree() {
            let idx = self.get_left_index_and_resize();
            let left = self.tree.inner.get_mut(idx).unwrap();
            *left = Some(elem);
            None
        } else {
            Some(elem)
        }
    }

    fn insert_as_right(&mut self, elem: Self::Elem) -> Option<Self::Elem> {
        if self.right_mut().is_none() && !self.is_empty_subtree() {
            let idx = self.get_right_index_and_resize();
            let right = self.tree.inner.get_mut(idx).unwrap();
            *right = Some(elem);
            None
        } else {
            Some(elem)
        }
    }

    fn append_left(&mut self, other: &mut Self) {
        if self.left_mut().is_some() {
            panic!("Left subtree is non-empty!")
        } else {
            let base = left_index(self.current);
            for (dst, src) in other.in_order_index_iter().enumerate() {
                let dst = in_order_index(base, dst);
                let src_node = other.get_node_and_resize(src);
                let dst_node = self.get_node_and_resize(dst);
                *dst_node = src_node.take();
            }
        }
    }

    fn append_right(&mut self, other: &mut Self) {
        if self.right_mut().is_some() {
            panic!("Right subtree is non-empty!")
        } else {
            let base = right_index(self.current);
            for (dst, src) in other.in_order_index_iter().enumerate() {
                let dst = in_order_index(base, dst);
                let src_node = other.get_node_and_resize(src);
                let dst_node = self.get_node_and_resize(dst);
                *dst_node = src_node.take();
            }
        }
    }

    fn into_inner(self) -> Option<Self::Elem> {
        if self.is_empty_subtree() {
            None
        } else {
            self.tree.inner.get_mut(self.current).unwrap().take()
        }
    }
}

impl<'a, T> BinTreeNodeMut<'a, VecBinaryTree<T>> for CursorMut<'a, T> {
    fn new(tree: &'a mut VecBinaryTree<T>) -> Self {
        Self { current: 0, tree }
    }

    fn take_left(&mut self) -> Option<VecBinaryTree<T>> {
        if self.is_empty_subtree() {
            None
        } else {
            let mut tree = VecBinaryTree::new();
            let mut cursor: CursorMut<_> = tree.cursor_mut();
            let iter = InOrderIndexIter::new(left_index(self.current), self.tree.inner.len());
            for (dst, src) in iter.enumerate() {
                if src < self.tree.inner.len() {
                    let src_node = self.get_node_and_resize(src);
                    let dst_node = cursor.get_node_and_resize(dst);
                    *dst_node = src_node.take();
                } else {
                    break;
                }
            }
            Some(tree)
        }
    }

    fn take_right(&mut self) -> Option<VecBinaryTree<T>> {
        if self.is_empty_subtree() {
            None
        } else {
            let mut tree = VecBinaryTree::new();
            let mut cursor: CursorMut<_> = tree.cursor_mut();
            let iter = InOrderIndexIter::new(right_index(self.current), self.tree.inner.len());
            for (dst, src) in iter.enumerate() {
                if src < self.tree.inner.len() {
                    let src_node = self.get_node_and_resize(src);
                    let dst_node = cursor.get_node_and_resize(dst);
                    *dst_node = src_node.take();
                } else {
                    break;
                }
            }
            Some(tree)
        }
    }
}

impl<'a, T> BinTree<Cursor<'a, T>> for CursorMut<'a, T> {
    type Elem = T;
}

impl<'a, T> BinTreeMut<Cursor<'a, T>, CursorMut<'a, T>> for CursorMut<'a, T> {}
