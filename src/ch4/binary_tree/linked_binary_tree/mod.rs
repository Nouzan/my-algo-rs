pub mod cursor;

use super::{BinTree, BinTreeCursor, BinTreeMut};

type Link<T> = Option<Box<Node<T>>>;

/// 链式二叉树结点.
struct Node<T> {
    left: Link<T>,
    right: Link<T>,
    /// 结点内容. 这里的`Option`是为了实现哨兵结点，所有非哨兵结点`elem`均为`Some`.
    elem: Option<T>,
}

/// 链式二叉树.
/// 带哨兵根结点，根结点是它的左孩子，根结点的后代都是非哨兵结点.
pub struct LinkedBinaryTree<T> {
    root: Node<T>,
}

impl<T> Default for LinkedBinaryTree<T> {
    fn default() -> Self {
        Self {
            root: Node {
                left: None,
                right: None,
                elem: None,
            },
        }
    }
}

impl<T: PartialEq> PartialEq for LinkedBinaryTree<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cursor().as_ref() == other.cursor().as_ref()
    }
}

impl<T: PartialOrd> PartialOrd for LinkedBinaryTree<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self.cursor().as_ref(), other.cursor().as_ref()) {
            (Some(lc), Some(rc)) => lc.partial_cmp(rc),
            _ => None,
        }
    }
}

impl<T> LinkedBinaryTree<T> {
    pub fn new() -> Self {
        Self::default()
    }

    fn replace_root_node(&mut self, node: Link<T>) -> Link<T> {
        let old = self.root.left.take();
        self.root.left = node;
        old
    }
}

impl<T> BinTree for LinkedBinaryTree<T> {
    type Elem = T;
    type Cursor<'a, E: 'a> = cursor::Cursor<'a, E>;

    fn cursor(&self) -> Self::Cursor<'_, Self::Elem> {
        cursor::Cursor::new(self)
    }
}

impl<T: 'static> BinTreeMut for LinkedBinaryTree<T> {
    type CursorMut<'a> = cursor::CursorMut<'a, T>;

    fn cursor_mut(&mut self) -> Self::CursorMut<'_> {
        cursor::CursorMut::new(self)
    }
}

#[cfg(test)]
mod test {
    use super::super::{BinTree, BinTreeCursor, BinTreeCursorExt, BinTreeCursorMut};
    use super::*;

    #[test]
    fn test_linked_binary_tree_basic() {
        let mut tree = LinkedBinaryTree::new();
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
        let mut tree = LinkedBinaryTree::new();
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
