pub mod cursor;
pub mod iter;

use super::{BinTree, BinTreeMut};
use cursor::{Cursor, CursorMut};
use iter::InOrderIter;

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
        self.inner.get(index).map_or(None, |elem| elem.as_ref())
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.inner.get_mut(index).map_or(None, |elem| elem.as_mut())
    }

    pub fn new() -> Self {
        Self::default()
    }
}

impl<T> BinTree for VecBinaryTree<T> {
    type Elem = T;
    type Node<'a, E: 'a> = Cursor<'a, E>;

    fn cursor<'a>(&'a self) -> Self::Node<'a, Self::Elem> {
        Cursor {
            current: 0,
            tree: self,
        }
    }

    fn in_order_iter<'a>(&'a self) -> Box<(dyn Iterator<Item = &'a T> + 'a)> {
        Box::new(InOrderIter::new(0, self))
    }
}

impl<T> BinTreeMut for VecBinaryTree<T> {
    type NodeMut<'a, E: 'a> = CursorMut<'a, E>;

    fn cursor_mut<'a>(&'a mut self) -> Self::NodeMut<'a, Self::Elem> {
        CursorMut {
            current: 0,
            tree: self,
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::{BinTreeNode, BinTreeNodeMut};
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
        for elem in tree.in_order_iter() {
            print!("{} ", elem);
        }
        println!();
        let mut cursor = tree.cursor_mut();
        let mut right = cursor.take_right().unwrap();
        for elem in tree.in_order_iter() {
            print!("{} ", elem);
        }
        println!();
        for elem in right.in_order_iter() {
            print!("{} ", elem);
        }
        println!();
        let mut cursor = tree.cursor_mut();
        cursor.move_left();
        cursor.append_left(&mut right.cursor_mut());
        assert!(right.is_empty());
        for elem in tree.in_order_iter() {
            print!("{} ", elem);
        }
        println!();
    }
}
