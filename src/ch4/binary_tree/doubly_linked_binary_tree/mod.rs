pub mod cursor;

use super::{BinTree, BinTreeCursor, BinTreeMut, MoveParentBinTreeMut};
use std::marker::PhantomData;
use std::ptr::NonNull;

type NodePosi<T> = NonNull<Node<T>>;
type Link<T> = Option<NodePosi<T>>;

#[derive(Debug)]
struct Node<T> {
    parent: Link<T>,
    left: Link<T>,
    right: Link<T>,
    elem: Option<T>,
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Box<Self> {
        Box::new(Self {
            parent: None,
            left: None,
            right: None,
            elem: Some(elem),
        })
    }

    pub fn leak<'a>(boxed: Box<Self>) -> NonNull<Node<T>>
    where
        T: 'a,
    {
        NonNull::new(Box::leak(boxed)).unwrap()
    }
}

pub struct DoublyLinkedBinaryTree<T> {
    root: NodePosi<T>,
    marker: PhantomData<Box<Node<T>>>,
}

impl<T> Default for DoublyLinkedBinaryTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: PartialEq> PartialEq for DoublyLinkedBinaryTree<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cursor().as_ref() == other.cursor().as_ref()
    }
}

impl<T: PartialOrd> PartialOrd for DoublyLinkedBinaryTree<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self.cursor().as_ref(), other.cursor().as_ref()) {
            (Some(lc), Some(rc)) => lc.partial_cmp(rc),
            _ => None,
        }
    }
}

impl<T> DoublyLinkedBinaryTree<T> {
    pub fn new() -> Self {
        let node = Box::new(Node {
            parent: None,
            left: None,
            right: None,
            elem: None,
        });
        Self {
            root: Node::leak(node),
            marker: PhantomData::default(),
        }
    }

    unsafe fn replace_root_node(&mut self, new: Link<T>) -> Link<T> {
        let link = if let Some(mut posi) = new {
            posi.as_mut().parent = Some(self.root);
            self.root.as_mut().left.replace(posi)
        } else {
            self.root.as_mut().left.take()
        };
        if let Some(mut old) = link {
            old.as_mut().parent = None;
        }
        link
    }

    fn into_root(mut self) -> Option<Box<Node<T>>> {
        unsafe {
            self.root
                .as_mut()
                .left
                .take()
                .map(|mut posi| Box::from_raw(posi.as_mut()))
        }
    }
}

impl<T> Drop for DoublyLinkedBinaryTree<T> {
    fn drop(&mut self) {
        unsafe {
            let mut node = Box::from_raw(self.root.as_ptr());
            if let Some(root) = node.left.take() {
                let root = Box::from_raw(root.as_ptr());
                if let Some(left) = root.left {
                    let mut tree = Self::new();
                    tree.replace_root_node(Some(left));
                }
                if let Some(right) = root.right {
                    let mut tree = Self::new();
                    tree.replace_root_node(Some(right));
                }
            }
        }
    }
}

impl<T> BinTree for DoublyLinkedBinaryTree<T> {
    type Elem = T;
    type Cursor<'a, E: 'a> = cursor::Cursor<'a, E>;

    fn cursor(&self) -> Self::Cursor<'_, Self::Elem> {
        cursor::Cursor::new(self)
    }
}

impl<T: 'static> BinTreeMut for DoublyLinkedBinaryTree<T> {
    type CursorMut<'a> = cursor::CursorMut<'a, T>;

    fn cursor_mut(&mut self) -> Self::CursorMut<'_> {
        cursor::CursorMut::new(self)
    }
}

impl<T: 'static> MoveParentBinTreeMut for DoublyLinkedBinaryTree<T> {
    type CursorMut<'a> = cursor::CursorMut<'a, T>;

    fn cursor_mut(&mut self) -> Self::CursorMut<'_> {
        cursor::CursorMut::new(self)
    }
}

#[cfg(test)]
mod test {
    use super::DoublyLinkedBinaryTree;
    use crate::ch4::{BinTree, BinTreeCursor, BinTreeCursorExt, BinTreeCursorMut, BinTreeMut};

    #[test]
    fn test_drop() {
        #[derive(Debug)]
        struct Foo(usize);

        impl Drop for Foo {
            fn drop(&mut self) {
                println!("drop: {:?}", self)
            }
        }

        let mut tree = DoublyLinkedBinaryTree::new();
        let mut cursor = tree.cursor_mut();
        cursor.insert_as_root(Foo(1));
        cursor.insert_as_left(Foo(2));
        cursor.insert_as_right(Foo(3));
        cursor.move_left();
        cursor.insert_as_left(Foo(4));
        cursor.insert_as_right(Foo(5));
    }

    #[test]
    fn test_linked_binary_tree_basic() {
        let mut tree = DoublyLinkedBinaryTree::new();
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
        let mut tree = DoublyLinkedBinaryTree::new();
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
