pub mod cursor;
pub mod iter;

use super::{BinTree, BinTreeCursor, BinTreeMut, MoveParentBinTreeMut};
use cursor::{Cursor, CursorMut};

/// 基于`Vec`实现的二叉顺序树.
pub struct VecBinaryTree<T> {
    pub(crate) inner: Vec<Option<T>>,
}

impl<T> Default for VecBinaryTree<T> {
    fn default() -> Self {
        Self { inner: Vec::new() }
    }
}

impl<T: PartialEq> PartialEq for VecBinaryTree<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cursor().as_ref() == other.cursor().as_ref()
    }
}

impl<T: PartialOrd> PartialOrd for VecBinaryTree<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self.cursor().as_ref(), other.cursor().as_ref()) {
            (Some(lc), Some(rc)) => lc.partial_cmp(rc),
            _ => None,
        }
    }
}

impl<T> VecBinaryTree<T> {
    fn get(&self, index: usize) -> Option<&T> {
        self.inner.get(index).and_then(|elem| elem.as_ref())
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.inner.get_mut(index).and_then(|elem| elem.as_mut())
    }

    pub fn new() -> Self {
        Self::default()
    }
}

impl<T> BinTree for VecBinaryTree<T> {
    type Elem = T;
    type Cursor<'a, E: 'a> = Cursor<'a, E>;

    fn cursor(&self) -> Self::Cursor<'_, Self::Elem> {
        Cursor::new(self)
    }
}

impl<T: 'static> BinTreeMut for VecBinaryTree<T> {
    type CursorMut<'a> = CursorMut<'a, T>;

    fn cursor_mut(&mut self) -> Self::CursorMut<'_> {
        CursorMut::new(self)
    }
}

impl<T: 'static> MoveParentBinTreeMut for VecBinaryTree<T> {
    type MoveParentCursorMut<'a> = CursorMut<'a, T>;

    fn move_parent_cursor_mut(&mut self) -> Self::MoveParentCursorMut<'_> {
        CursorMut::new(self)
    }
}

#[cfg(test)]
mod test {
    use super::super::{BinTree, BinTreeCursor, BinTreeCursorExt, BinTreeCursorMut, BinTreeMut};
    use super::VecBinaryTree;

    #[test]
    fn test_vec_binary_tree_basic() {
        let mut tree = VecBinaryTree::new();
        let mut cursor = tree.cursor_mut();
        cursor.insert_as_root(0);
        cursor.insert_as_left(1);
        cursor.insert_as_right(2);
        cursor.move_right();
        cursor.insert_as_left(3);
        cursor.move_left();
        cursor.insert_as_left(4);
        cursor.insert_as_right(5);
        assert_eq!(
            tree.cursor().in_order_iter().copied().collect::<Vec<_>>(),
            [0, 1, 2, 3, 4, 5]
        );
        let mut cursor = tree.cursor_mut();
        let right = cursor.take_right().unwrap();
        assert_eq!(
            tree.cursor().in_order_iter().copied().collect::<Vec<_>>(),
            [0, 1]
        );
        assert_eq!(
            right.cursor().in_order_iter().copied().collect::<Vec<_>>(),
            [2, 3, 4, 5]
        );
        let mut cursor = tree.cursor_mut();
        cursor.move_left();
        cursor.append_left(right);
        assert_eq!(
            tree.cursor().in_order_iter().copied().collect::<Vec<_>>(),
            [0, 1, 2, 3, 4, 5]
        );
        let mut cursor = tree.cursor_mut();
        cursor.insert_as_right(6);
        assert_eq!(
            tree.cursor().in_order_iter().copied().collect::<Vec<_>>(),
            [0, 1, 6, 2, 3, 4, 5]
        );
        assert_eq!(
            tree.cursor().pre_order_iter().copied().collect::<Vec<_>>(),
            [0, 1, 2, 3, 4, 5, 6]
        );
        assert_eq!(
            tree.cursor().mid_order_iter().copied().collect::<Vec<_>>(),
            [4, 3, 5, 2, 1, 0, 6]
        );
        assert_eq!(
            tree.cursor().post_order_iter().copied().collect::<Vec<_>>(),
            [4, 5, 3, 2, 1, 6, 0]
        );
        let mut cursor = tree.cursor_mut();
        cursor.move_right();
        assert_eq!(
            cursor
                .cursor()
                .post_order_iter()
                .copied()
                .collect::<Vec<_>>(),
            [6]
        );
    }

    #[test]
    fn test_post_order_iter() {
        let mut tree = VecBinaryTree::new();
        assert_eq!(
            tree.cursor().post_order_iter().copied().collect::<Vec<_>>(),
            []
        );
        let mut cursor = tree.cursor_mut();
        cursor.insert_as_root(1);
        cursor.insert_as_left(2);
        cursor.insert_as_right(3);
        cursor.move_left();
        cursor.insert_as_left(4);
        cursor.insert_as_right(5);
        cursor = tree.cursor_mut();
        cursor.move_right();
        cursor.insert_as_left(6);
        cursor.move_left();
        cursor.insert_as_left(7);
        cursor.insert_as_right(8);
        assert_eq!(
            tree.cursor().post_order_iter().copied().collect::<Vec<_>>(),
            [4, 5, 2, 7, 8, 6, 3, 1]
        );
    }
}
