pub mod cursor;
pub mod iter;

use super::{BinTree, BinTreeMut};
use cursor::{Cursor, CursorMut};

pub struct VecBinaryTree<T> {
    inner: Vec<Option<T>>,
}

impl<T> Default for VecBinaryTree<T> {
    fn default() -> Self {
        Self { inner: Vec::new() }
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

impl<'a, T> BinTree<Cursor<'a, T>> for VecBinaryTree<T> {
    type Elem = T;
}

impl<'a, T> BinTreeMut<Cursor<'a, T>, CursorMut<'a, T>> for VecBinaryTree<T> {}

#[cfg(test)]
mod test {
    use super::super::{BaseNode, BaseNodeExt, BaseNodeMut, BinTree};
    use super::*;

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
        for elem in tree.cursor().in_order_iter() {
            print!("{} ", elem);
        }
        println!();
        let mut cursor = tree.cursor_mut();
        let mut right = cursor.take_right().unwrap();
        for elem in tree.cursor().in_order_iter() {
            print!("{} ", elem);
        }
        println!();
        for elem in right.cursor().in_order_iter() {
            print!("{} ", elem);
        }
        println!();
        let mut cursor = tree.cursor_mut();
        cursor.move_left();
        cursor.append_left(&mut right.cursor_mut());
        assert!(right.is_empty());
        for elem in tree.cursor().in_order_iter() {
            print!("{} ", elem);
        }
        println!();
        let mut cursor = tree.cursor_mut();
        cursor.insert_as_right(6);
        // cursor.move_left();
        for elem in tree.cursor().in_order_iter() {
            print!("{} ", elem);
        }
        println!();
        for elem in tree.cursor().pre_order_iter() {
            print!("{} ", elem);
        }
        println!();
        for elem in tree.cursor().mid_order_iter() {
            print!("{} ", elem);
        }
        println!();
        for elem in tree.cursor().post_order_iter() {
            print!("{} ", elem);
        }
        println!();

        let cursor = tree.cursor_mut();
        // let cursor = cursor.cursor();
        // let (left, right) = cursor.split();

        for elem in cursor.cursor().post_order_iter() {
            print!("{} ", elem);
        }
        println!();
    }
}
