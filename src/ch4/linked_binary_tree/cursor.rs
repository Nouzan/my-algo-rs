use super::super::{BaseNode, BaseNodeMut, BinTree, BinTreeNode, BinTreeNodeMut};
use super::{LinkedBinaryTree, Node};

/// 链式二叉树只读游标.
pub struct Cursor<'a, T> {
    /// 当前结点，`None`表示当前结点不存在.
    current: Option<&'a Node<T>>,
}

impl<'a, T> Clone for Cursor<'a, T> {
    fn clone(&self) -> Self {
        Self {
            current: self.current,
        }
    }
}

impl<'a, T> Cursor<'a, T> {
    fn is_left(&self, other: &Self) -> bool {
        self.current
            .and_then(|node| node.left.as_deref())
            .zip(other.current)
            .map_or(false, |(lhs, rhs)| lhs as *const _ == rhs as *const _)
    }

    fn is_right(&self, other: &Self) -> bool {
        self.current
            .and_then(|node| node.right.as_deref())
            .zip(other.current)
            .map_or(false, |(lhs, rhs)| lhs as *const _ == rhs as *const _)
    }
}

impl<'a, T> BaseNode<'a> for Cursor<'a, T> {
    type Elem = T;

    fn as_ref(&self) -> Option<&Self::Elem> {
        self.current.map(|node| node.elem.as_ref().unwrap())
    }

    fn left(&self) -> Option<&Self::Elem> {
        self.current
            .and_then(|node| node.left.as_ref().map(|node| node.elem.as_ref().unwrap()))
    }

    fn right(&self) -> Option<&Self::Elem> {
        self.current
            .and_then(|node| node.right.as_ref().map(|node| node.elem.as_ref().unwrap()))
    }

    fn is_parent(&self, other: &Self) -> bool {
        other.is_left(self) || other.is_right(self)
    }

    fn move_left(&mut self) {
        self.current = self.current.take().and_then(|node| node.left.as_deref())
    }

    fn move_right(&mut self) {
        self.current = self.current.take().and_then(|node| node.right.as_deref())
    }

    fn into_ref(self) -> Option<&'a Self::Elem>
    where
        Self: Sized,
    {
        self.current.map(|node| node.elem.as_ref().unwrap())
    }
}

impl<'a, T> BinTreeNode<'a, LinkedBinaryTree<T>> for Cursor<'a, T> {
    fn new(tree: &'a LinkedBinaryTree<T>) -> Self {
        Self {
            current: tree.root.left.as_deref(),
        }
    }
}

impl<'a, T> BinTreeNode<'a, CursorMut<'a, T>> for Cursor<'a, T> {
    fn new(tree: &'a CursorMut<'a, T>) -> Self {
        Self {
            current: if tree.is_left_child {
                tree.parent.as_ref().unwrap().left.as_deref()
            } else {
                tree.parent.as_ref().unwrap().right.as_deref()
            },
        }
    }
}

/// 链式二叉树可变游标.
pub struct CursorMut<'a, T> {
    /// 指向当前结点的父母结点.
    /// `parent`必不为`None`，这里用`Option`是为了便于进行所有权操作.
    parent: Option<&'a mut Node<T>>,
    /// 当前结点是否为父母结点的左孩子(若不是，则为右孩子).
    is_left_child: bool,
}

impl<'a, T> CursorMut<'a, T> {
    fn current_mut(&mut self) -> Option<&mut Node<T>> {
        if self.is_left_child {
            self.parent.as_mut().unwrap().left.as_deref_mut()
        } else {
            self.parent.as_mut().unwrap().right.as_deref_mut()
        }
    }

    fn current(&self) -> Option<&Node<T>> {
        if self.is_left_child {
            self.parent.as_ref().unwrap().left.as_deref()
        } else {
            self.parent.as_ref().unwrap().right.as_deref()
        }
    }

    fn is_left(&self, other: &Self) -> bool {
        self.current()
            .as_ref()
            .and_then(|node| node.left.as_deref())
            .zip(other.current().as_deref())
            .map_or(false, |(lhs, rhs)| lhs as *const _ == rhs as *const _)
    }

    fn is_right(&self, other: &Self) -> bool {
        self.current()
            .as_ref()
            .and_then(|node| node.right.as_deref())
            .zip(other.current().as_deref())
            .map_or(false, |(lhs, rhs)| lhs as *const _ == rhs as *const _)
    }
}

impl<'a, T> BaseNode<'a> for CursorMut<'a, T> {
    type Elem = T;

    fn as_ref(&self) -> Option<&Self::Elem> {
        self.current()
            .as_ref()
            .map(|node| node.elem.as_ref().unwrap())
    }

    fn left(&self) -> Option<&Self::Elem> {
        self.current()
            .as_ref()
            .and_then(|node| node.left.as_ref().map(|node| node.elem.as_ref().unwrap()))
    }

    fn right(&self) -> Option<&Self::Elem> {
        self.current()
            .as_ref()
            .and_then(|node| node.right.as_ref().map(|node| node.elem.as_ref().unwrap()))
    }

    fn is_parent(&self, other: &Self) -> bool {
        other.is_left(self) || other.is_right(self)
    }

    fn move_left(&mut self) {
        if let Some(current) = {
            if self.is_left_child {
                self.parent.take().unwrap().left.as_deref_mut()
            } else {
                self.parent.take().unwrap().right.as_deref_mut()
            }
        } {
            self.is_left_child = true;
            self.parent = Some(current);
        }
    }

    fn move_right(&mut self) {
        if let Some(current) = {
            if self.is_left_child {
                self.parent.take().unwrap().left.as_deref_mut()
            } else {
                self.parent.take().unwrap().right.as_deref_mut()
            }
        } {
            self.is_left_child = false;
            self.parent = Some(current);
        }
    }

    fn into_ref(self) -> Option<&'a Self::Elem>
    where
        Self: Sized,
    {
        if self.is_left_child {
            self.parent
                .unwrap()
                .left
                .as_ref()
                .map(|node| node.elem.as_ref().unwrap())
        } else {
            self.parent
                .unwrap()
                .right
                .as_ref()
                .map(|node| node.elem.as_ref().unwrap())
        }
    }
}

impl<'a, T> BaseNodeMut<'a> for CursorMut<'a, T> {
    type SubTree = LinkedBinaryTree<T>;

    fn as_mut(&mut self) -> Option<&mut Self::Elem> {
        if self.is_left_child {
            self.parent
                .as_mut()
                .unwrap()
                .left
                .as_mut()
                .map(|node| node.elem.as_mut().unwrap())
        } else {
            self.parent
                .as_mut()
                .unwrap()
                .right
                .as_mut()
                .map(|node| node.elem.as_mut().unwrap())
        }
    }

    fn left_mut(&mut self) -> Option<&mut Self::Elem> {
        if self.is_left_child {
            self.parent
                .as_mut()
                .unwrap()
                .left
                .as_mut()
                .and_then(|node| node.left.as_mut().map(|node| node.elem.as_mut().unwrap()))
        } else {
            self.parent
                .as_mut()
                .unwrap()
                .right
                .as_mut()
                .and_then(|node| node.left.as_mut().map(|node| node.elem.as_mut().unwrap()))
        }
    }

    fn right_mut(&mut self) -> Option<&mut Self::Elem> {
        if self.is_left_child {
            self.parent
                .as_mut()
                .unwrap()
                .left
                .as_mut()
                .and_then(|node| node.right.as_mut().map(|node| node.elem.as_mut().unwrap()))
        } else {
            self.parent
                .as_mut()
                .unwrap()
                .right
                .as_mut()
                .and_then(|node| node.right.as_mut().map(|node| node.elem.as_mut().unwrap()))
        }
    }

    fn insert_as_root(&mut self, elem: Self::Elem) -> Option<Self::Elem> {
        if self.is_empty_subtree() {
            let node = Some(Box::new(Node {
                left: None,
                right: None,
                elem: Some(elem),
            }));
            if self.is_left_child {
                self.parent.as_mut().unwrap().left = node
            } else {
                self.parent.as_mut().unwrap().right = node
            }

            None
        } else {
            Some(elem)
        }
    }

    fn insert_as_left(&mut self, elem: Self::Elem) -> Option<Self::Elem> {
        if let Some(current) = self.current_mut() {
            if current.left.is_none() {
                current.left = Some(Box::new(Node {
                    left: None,
                    right: None,
                    elem: Some(elem),
                }));
                None
            } else {
                Some(elem)
            }
        } else {
            Some(elem)
        }
    }

    fn insert_as_right(&mut self, elem: Self::Elem) -> Option<Self::Elem> {
        if let Some(current) = self.current_mut() {
            if current.right.is_none() {
                current.right = Some(Box::new(Node {
                    left: None,
                    right: None,
                    elem: Some(elem),
                }));
                None
            } else {
                Some(elem)
            }
        } else {
            Some(elem)
        }
    }

    fn take_left(&mut self) -> Option<Self::SubTree>
    where
        Self::SubTree: Sized,
    {
        if let Some(current) = self.current_mut() {
            let node = current.left.take();
            let mut tree = LinkedBinaryTree::default();
            tree.replace_root_node(node);
            Some(tree)
        } else {
            None
        }
    }

    fn take_right(&mut self) -> Option<Self::SubTree>
    where
        Self::SubTree: Sized,
    {
        if let Some(current) = self.current_mut() {
            let node = current.right.take();
            let mut tree = LinkedBinaryTree::default();
            tree.replace_root_node(node);
            Some(tree)
        } else {
            None
        }
    }

    fn append_left(&mut self, other: &mut Self) {
        if self.left().is_some() {
            panic!("左子树不为空!");
        } else {
            let node = if other.is_left_child {
                other.parent.as_mut().unwrap().left.take()
            } else {
                other.parent.as_mut().unwrap().right.take()
            };
            let current = self.current_mut().unwrap();
            current.left = node;
        }
    }

    fn append_right(&mut self, other: &mut Self) {
        if self.right().is_some() {
            panic!("左子树不为空!");
        } else {
            let node = if other.is_left_child {
                other.parent.as_mut().unwrap().left.take()
            } else {
                other.parent.as_mut().unwrap().right.take()
            };
            let current = self.current_mut().unwrap();
            current.right = node;
        }
    }

    fn into_inner(self) -> Option<Self::Elem> {
        let node = if self.is_left_child {
            self.parent.unwrap().left.take()
        } else {
            self.parent.unwrap().right.take()
        };
        node.and_then(|node| node.elem)
    }

    fn into_mut(self) -> Option<&'a mut Self::Elem>
    where
        Self: Sized,
    {
        if self.is_left_child {
            self.parent
                .unwrap()
                .left
                .as_mut()
                .map(|node| node.elem.as_mut().unwrap())
        } else {
            self.parent
                .unwrap()
                .right
                .as_mut()
                .map(|node| node.elem.as_mut().unwrap())
        }
    }
}

impl<'a, T> BinTreeNodeMut<'a, LinkedBinaryTree<T>> for CursorMut<'a, T> {
    fn new(tree: &'a mut LinkedBinaryTree<T>) -> Self {
        Self {
            parent: Some(&mut tree.root),
            is_left_child: true,
        }
    }
}

impl<'a, T> BinTree<Cursor<'a, T>> for CursorMut<'a, T> {
    type Elem = T;
}
