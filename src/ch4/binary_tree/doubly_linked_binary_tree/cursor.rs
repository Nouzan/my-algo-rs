use super::{DoublyLinkedBinaryTree, Link, Node, NodePosi};
use crate::ch4::{BinTree, BinTreeCursor, BinTreeCursorMut, MoveParentCursor, MoveParentCursorMut};

pub struct Cursor<'a, T> {
    parent: NodePosi<T>,
    is_left: bool,
    tree: &'a DoublyLinkedBinaryTree<T>,
}

pub struct CursorMut<'a, T> {
    parent: NodePosi<T>,
    is_left: bool,
    tree: &'a mut DoublyLinkedBinaryTree<T>,
}

unsafe fn parent<T>(link: Link<T>) -> Link<T> {
    link.and_then(|posi| posi.as_ref().parent)
}

unsafe fn left<T>(link: Link<T>) -> Link<T> {
    link.and_then(|posi| posi.as_ref().left)
}

unsafe fn right<T>(link: Link<T>) -> Link<T> {
    link.and_then(|posi| posi.as_ref().right)
}

unsafe fn elem<'a, T>(link: Link<T>) -> Option<&'a T> {
    link.and_then(|posi| (*posi.as_ptr()).elem.as_ref())
}

unsafe fn elem_mut<'a, T>(link: Link<T>) -> Option<&'a mut T> {
    link.and_then(|posi| (*posi.as_ptr()).elem.as_mut())
}

impl<'a, T> Cursor<'a, T> {
    pub fn new(tree: &'a DoublyLinkedBinaryTree<T>) -> Self {
        Self {
            parent: tree.root,
            is_left: true,
            tree,
        }
    }

    fn current_link(&self) -> Link<T> {
        unsafe {
            if self.is_left {
                self.parent.as_ref().left
            } else {
                self.parent.as_ref().right
            }
        }
    }

    fn is_root(&self) -> bool {
        self.parent == self.tree.root
    }
}

impl<'a, T> Clone for Cursor<'a, T> {
    fn clone(&self) -> Self {
        Self {
            parent: self.parent,
            is_left: self.is_left,
            tree: self.tree,
        }
    }
}

impl<'a, T> BinTreeCursor<'a> for Cursor<'a, T> {
    type Elem = T;

    fn is_parent(&self, other: &Self) -> bool {
        other.current_link() == Some(self.parent)
    }

    fn as_ref(&self) -> Option<&Self::Elem> {
        unsafe { elem(self.current_link()) }
    }

    fn left(&self) -> Option<&Self::Elem> {
        unsafe { elem(left(self.current_link())) }
    }

    fn right(&self) -> Option<&Self::Elem> {
        unsafe { elem(right(self.current_link())) }
    }

    fn move_left(&mut self) {
        if let Some(posi) = self.current_link() {
            self.parent = posi;
            self.is_left = true;
        }
    }

    fn move_right(&mut self) {
        if let Some(posi) = self.current_link() {
            self.parent = posi;
            self.is_left = false;
        }
    }

    fn into_ref(self) -> Option<&'a Self::Elem>
    where
        Self: Sized,
    {
        unsafe { elem(self.current_link()) }
    }
}

impl<'a, T> MoveParentCursor<'a> for Cursor<'a, T> {
    fn move_parent(&mut self) {
        if !self.is_root() {
            // 非根则必有父母.
            unsafe {
                let current = Some(self.parent);
                self.parent = parent(current).unwrap();
                self.is_left = !((self.current_link() == current) ^ self.is_left)
            }
        }
    }

    fn parent(&self) -> Option<&Self::Elem> {
        if !self.is_root() {
            unsafe { elem(Some(self.parent)) }
        } else {
            None
        }
    }
}

impl<'a, T> CursorMut<'a, T> {
    pub fn new(tree: &'a mut DoublyLinkedBinaryTree<T>) -> Self {
        Self {
            parent: tree.root,
            is_left: true,
            tree,
        }
    }

    fn current_link(&self) -> Link<T> {
        unsafe {
            if self.is_left {
                self.parent.as_ref().left
            } else {
                self.parent.as_ref().right
            }
        }
    }

    fn is_root(&self) -> bool {
        self.parent == self.tree.root
    }
}

impl<'a, T> BinTreeCursor<'a> for CursorMut<'a, T> {
    type Elem = T;

    fn is_parent(&self, other: &Self) -> bool {
        other.current_link() == Some(self.parent)
    }

    fn as_ref(&self) -> Option<&Self::Elem> {
        unsafe { elem(self.current_link()) }
    }

    fn left(&self) -> Option<&Self::Elem> {
        unsafe { elem(left(self.current_link())) }
    }

    fn right(&self) -> Option<&Self::Elem> {
        unsafe { elem(right(self.current_link())) }
    }

    fn move_left(&mut self) {
        if let Some(posi) = self.current_link() {
            self.parent = posi;
            self.is_left = true;
        }
    }

    fn move_right(&mut self) {
        if let Some(posi) = self.current_link() {
            self.parent = posi;
            self.is_left = false;
        }
    }

    fn into_ref(self) -> Option<&'a Self::Elem>
    where
        Self: Sized,
    {
        unsafe { elem(self.current_link()) }
    }
}

impl<'a, T> MoveParentCursor<'a> for CursorMut<'a, T> {
    fn move_parent(&mut self) {
        if !self.is_root() {
            // 非根则必有父母.
            unsafe {
                let current = Some(self.parent);
                self.parent = parent(current).unwrap();
                self.is_left = !((self.current_link() == current) ^ self.is_left)
            }
        }
    }

    fn parent(&self) -> Option<&Self::Elem> {
        if !self.is_root() {
            unsafe { elem(Some(self.parent)) }
        } else {
            None
        }
    }
}

impl<'a, T> BinTreeCursorMut<'a> for CursorMut<'a, T> {
    type SubTree = DoublyLinkedBinaryTree<T>;

    fn as_mut(&mut self) -> Option<&mut Self::Elem> {
        unsafe { elem_mut(self.current_link()) }
    }

    fn left_mut(&mut self) -> Option<&mut Self::Elem> {
        unsafe { elem_mut(left(self.current_link())) }
    }

    fn right_mut(&mut self) -> Option<&mut Self::Elem> {
        unsafe { elem_mut(right(self.current_link())) }
    }

    fn move_succ_and_split_mut(&mut self) -> (Option<&mut Self::Elem>, Option<&mut Self::Elem>) {
        // Safety: `move_left`和`move_right`的确不会改变树，也不会移动树的结点.
        unsafe { self.move_succ_and_split_mut_unchecked() }
    }

    fn insert_as_root(&mut self, elem: Self::Elem) -> Option<Self::Elem> {
        if self.is_empty_subtree() {
            let mut node = Node::new(elem);
            node.parent = Some(self.parent);
            unsafe {
                if self.is_left {
                    self.parent.as_mut().left = Some(Node::leak(node));
                } else {
                    self.parent.as_mut().right = Some(Node::leak(node));
                }
            }
            None
        } else {
            Some(elem)
        }
    }

    fn insert_as_left(&mut self, elem: Self::Elem) -> Option<Self::Elem> {
        if let Some(mut current) = self.current_link() {
            let mut node = Node::new(elem);
            node.parent = Some(current);
            unsafe {
                current.as_mut().left = Some(Node::leak(node));
            }
            None
        } else {
            None
        }
    }

    fn insert_as_right(&mut self, elem: Self::Elem) -> Option<Self::Elem> {
        if let Some(mut current) = self.current_link() {
            let mut node = Node::new(elem);
            node.parent = Some(current);
            unsafe {
                current.as_mut().right = Some(Node::leak(node));
            }
            None
        } else {
            None
        }
    }

    fn take(&mut self) -> Self::SubTree {
        let mut tree = DoublyLinkedBinaryTree::new();
        unsafe {
            if let Some(posi) = if self.is_left {
                self.parent.as_mut().left.take()
            } else {
                self.parent.as_mut().right.take()
            } {
                tree.replace_root_node(Some(posi));
            }
        }
        tree
    }

    fn append(&mut self, mut other: Self::SubTree) {
        if self.is_empty_subtree() {
            unsafe {
                let root = other.replace_root_node(None);
                if let Some(mut posi) = root {
                    posi.as_mut().parent = Some(self.parent);
                }
                if self.is_left {
                    self.parent.as_mut().left = root;
                } else {
                    self.parent.as_mut().right = root;
                }
            }
        } else {
            panic!("子树不为空!")
        }
    }

    fn take_left(&mut self) -> Option<Self::SubTree>
    where
        Self::SubTree: Sized,
    {
        unsafe {
            if let Some(mut posi) = self.current_link() {
                let mut tree = DoublyLinkedBinaryTree::new();
                let posi = posi.as_mut().left.take();
                tree.replace_root_node(posi);
                Some(tree)
            } else {
                None
            }
        }
    }

    fn take_right(&mut self) -> Option<Self::SubTree>
    where
        Self::SubTree: Sized,
    {
        unsafe {
            if let Some(mut posi) = self.current_link() {
                let mut tree = DoublyLinkedBinaryTree::new();
                let posi = posi.as_mut().right.take();
                tree.replace_root_node(posi);
                Some(tree)
            } else {
                None
            }
        }
    }

    fn append_left(&mut self, mut other: Self::SubTree) {
        if let Some(mut posi) = self.current_link() {
            unsafe {
                if left(Some(posi)).is_none() {
                    let root = other.replace_root_node(None);
                    if let Some(mut p) = root {
                        p.as_mut().parent = Some(posi);
                    }
                    posi.as_mut().left = root;
                } else {
                    panic!("左子树不为空!")
                }
            }
        } else {
            panic!("子树为空!")
        }
    }

    fn append_right(&mut self, mut other: Self::SubTree) {
        if let Some(mut posi) = self.current_link() {
            unsafe {
                if right(Some(posi)).is_none() {
                    let root = other.replace_root_node(None);
                    if let Some(mut p) = root {
                        p.as_mut().parent = Some(posi);
                    }
                    posi.as_mut().right = root;
                } else {
                    panic!("右子树不为空!")
                }
            }
        } else {
            panic!("子树为空!")
        }
    }

    fn into_mut(self) -> Option<&'a mut Self::Elem>
    where
        Self: Sized,
    {
        unsafe { elem_mut(self.current_link()) }
    }

    fn into_inner(mut self) -> Option<Self::Elem> {
        let tree = self.take();
        tree.into_root().and_then(|node| node.elem)
    }
}

impl<'a, T> MoveParentCursorMut<'a> for CursorMut<'a, T> {
    fn parent_mut(&mut self) -> Option<&mut Self::Elem> {
        if !self.is_root() {
            unsafe { elem_mut(Some(self.parent)) }
        } else {
            None
        }
    }
}

impl<'a, T> BinTree for CursorMut<'a, T> {
    type Elem = T;
    type Cursor<'b, E: 'b> = Cursor<'b, E>;

    fn cursor(&self) -> Self::Cursor<'_, Self::Elem> {
        Cursor {
            parent: self.parent,
            is_left: self.is_left,
            tree: self.tree,
        }
    }
}
