use super::super::{BinTree, BinTreeCursor, BinTreeCursorMut, BinTreeMut};
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

pub(super) const fn parent_index(index: usize) -> usize {
    (index - 1) / 2
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

impl<'a, T> BinTreeCursor<'a> for Cursor<'a, T> {
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

    fn is_parent(&self, other: &Self) -> bool {
        parent_index(self.current) == other.current
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

impl<'a, T> Cursor<'a, T> {
    pub fn new(tree: &'a VecBinaryTree<T>) -> Self {
        Self { current: 0, tree }
    }

    pub fn from_cursor_mut(cursor: &'a CursorMut<'a, T>) -> Self {
        Self {
            current: cursor.current,
            tree: cursor.tree,
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

impl<'a, T> BinTreeCursor<'a> for CursorMut<'a, T> {
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

    fn is_parent(&self, other: &Self) -> bool {
        parent_index(self.current) == other.current
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

impl<'a, T: 'static> BinTreeCursorMut<'a> for CursorMut<'a, T> {
    type SubTree = VecBinaryTree<T>;

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

    fn move_succ_and_split_mut(&mut self) -> (Option<&mut Self::Elem>, Option<&mut Self::Elem>) {
        // Safety: `move_left`和`move_right`的实现仅仅只是修改了`current: usize`的值，并未对树进行任何修改和移动.
        unsafe { self.move_succ_and_split_mut_unchecked() }
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

    fn append(&mut self, mut other: Self::SubTree) {
        let mut other = other.cursor_mut();
        if !self.is_empty_subtree() {
            panic!("Subtree is non-empty!");
        } else {
            let base = self.current;
            for (dst, src) in other.in_order_index_iter().enumerate() {
                let dst = in_order_index(base, dst);
                let src_node = other.get_node_and_resize(src);
                let dst_node = self.get_node_and_resize(dst);
                *dst_node = src_node.take();
            }
        }
    }

    fn append_left(&mut self, mut other: Self::SubTree) {
        let mut other = other.cursor_mut();
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

    fn append_right(&mut self, mut other: Self::SubTree) {
        let mut other = other.cursor_mut();
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

    fn take(&mut self) -> Self::SubTree {
        let mut tree = VecBinaryTree::new();
        let mut cursor: CursorMut<_> = tree.cursor_mut();
        let iter = InOrderIndexIter::new(self.current, self.tree.inner.len());
        for (dst, src) in iter.enumerate() {
            if src < self.tree.inner.len() {
                let src_node = self.get_node_and_resize(src);
                let dst_node = cursor.get_node_and_resize(dst);
                *dst_node = src_node.take();
            } else {
                break;
            }
        }
        tree
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

    fn into_mut(self) -> Option<&'a mut Self::Elem>
    where
        Self: Sized,
    {
        self.tree.get_mut(self.current)
    }

    fn into_inner(self) -> Option<Self::Elem> {
        if self.is_empty_subtree() {
            None
        } else {
            self.tree.inner.get_mut(self.current).unwrap().take()
        }
    }
}

impl<'a, T> CursorMut<'a, T> {
    pub fn new(tree: &'a mut VecBinaryTree<T>) -> Self {
        Self { current: 0, tree }
    }
}

impl<'a, T> BinTree for CursorMut<'a, T> {
    type Elem = T;
    type Cursor<'b, E: 'b> = Cursor<'b, E>;

    fn cursor(&self) -> Self::Cursor<'_, Self::Elem> {
        Cursor::from_cursor_mut(self)
    }
}

// impl<'a, T> BinTreeMut for CursorMut<'a, T> {
//     type CursorMut<'b, E: 'b, St> = CursorMut<'b, E>;

//     fn cursor_mut(&mut self) -> Self::CursorMut<'_, Self::Elem, Self> {
//         CursorMut {
//             current: self.current,
//             tree: self.tree,
//         }
//     }
// }

// unsafe impl<'a, T> SplitNodeMut<'a> for CursorMut<'a, T> {
//     fn split_mut(&mut self) -> (Option<Self>, Option<Self>)
//     where Self: Sized {
//         // 这里是安全的. 因为`CursorMut`(的公开方法)只能沿树下降，左、右子树只对其子树是可变的.
//         // 且左右子树不相交、祖先链则已被冻结，故不存在共享可变引用.
//         unsafe {
//             let ptr: *mut _ = self.tree;
//             let left = if self.left().is_some() {
//                 let mut next = Self {
//                     current: self.current,
//                     tree: &mut *ptr,
//                 };
//                 next.move_left();
//                 Some(next)
//             } else {
//                 None
//             };
//             let right = if self.right_mut().is_some() {
//                 let mut next = Self {
//                     current: self.current,
//                     tree: &mut *ptr,
//                 };
//                 next.move_right();
//                 Some(next)
//             } else {
//                 None
//             };
//             (left, right)
//         }
//     }
// }
