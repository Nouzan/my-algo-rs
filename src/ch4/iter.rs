use super::{BinTree, BinTreeNode};
use std::collections::VecDeque;
use std::marker::PhantomData;

/// 层序遍历迭代器结构.
pub struct InOrderIter<'a, Cursor, Tree> {
    queue: VecDeque<Cursor>,
    marker: PhantomData<&'a Tree>,
}

impl<'a, Cursor, Tree> InOrderIter<'a, Cursor, Tree> {
    pub fn new(root: Option<Cursor>) -> Self {
        let mut queue = VecDeque::new();
        if let Some(root) = root {
            queue.push_back(root);
        }
        InOrderIter {
            queue,
            marker: PhantomData::default(),
        }
    }
}

impl<'a, T: 'a, Tree, Cursor: BinTreeNode<'a, Tree, Elem = T> + Clone> Iterator
    for InOrderIter<'a, Cursor, Tree>
{
    type Item = &'a Cursor::Elem;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cursor) = self.queue.pop_front() {
            let (left, right) = cursor.split();
            if let Some(left) = left {
                self.queue.push_back(left);
            }
            if let Some(right) = right {
                self.queue.push_back(right);
            }
            cursor.into_ref()
        } else {
            None
        }
    }
}

/// 前序遍历迭代器结构.
pub struct PreOrderIter<'a, Cursor, Tree> {
    current: Option<Cursor>,
    stack: Vec<Cursor>,
    marker: PhantomData<&'a Tree>,
}

impl<'a, Cursor, Tree> PreOrderIter<'a, Cursor, Tree> {
    pub fn new(root: Option<Cursor>) -> Self {
        Self {
            current: root,
            stack: Vec::new(),
            marker: PhantomData::default(),
        }
    }
}

impl<'a, T: 'a, Tree, Cursor: BinTreeNode<'a, Tree, Elem = T> + Clone> Iterator
    for PreOrderIter<'a, Cursor, Tree>
{
    type Item = &'a Cursor::Elem;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current.take() {
            self.stack.push(current.clone());
            if current.left().is_some() {
                let mut next = current.clone();
                next.move_left();
                self.current = Some(next);
            } else {
                while let Some(mut old) = self.stack.pop() {
                    if old.right().is_some() {
                        old.move_right();
                        self.current = Some(old);
                        break;
                    }
                }
            }
            current.into_ref()
        } else {
            None
        }
    }
}

/// 中序遍历迭代器结构.
pub struct MidOrderIter<'a, Cursor, Tree> {
    current: Option<Cursor>,
    stack: Vec<Cursor>,
    marker: PhantomData<&'a Tree>,
}

impl<'a, Tree, Cursor: BinTreeNode<'a, Tree> + Clone> MidOrderIter<'a, Cursor, Tree> {
    fn push_left_chain(&mut self, current: &mut Cursor) {
        while !current.is_empty_subtree() {
            self.stack.push(current.clone());
            current.move_left();
        }
    }

    pub fn new(mut root: Cursor) -> Self {
        let mut iter = Self {
            current: None,
            stack: Vec::new(),
            marker: PhantomData::default(),
        };
        iter.push_left_chain(&mut root);
        iter.current = iter.stack.pop();
        iter
    }
}

impl<'a, T: 'a, Tree, Cursor: BinTreeNode<'a, Tree, Elem = T> + Clone> Iterator
    for MidOrderIter<'a, Cursor, Tree>
{
    type Item = &'a Cursor::Elem;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current.take() {
            let mut right = current.clone();
            right.move_right();
            self.push_left_chain(&mut right);
            self.current = self.stack.pop();
            current.into_ref()
        } else {
            None
        }
    }
}

/// 后序遍历迭代器.
pub struct PostOrderIter<'a, Cursor, Tree> {
    left_stack: Vec<Cursor>,
    right_stack: Vec<Cursor>,
    marker: PhantomData<&'a Tree>,
}

impl<'a, Tree, Cursor: BinTreeNode<'a, Tree> + Clone> PostOrderIter<'a, Cursor, Tree> {
    fn push_deep_most_chain(&mut self, current: &mut Cursor) {
        while !current.is_empty_subtree() {
            if current.left().is_some() {
                self.left_stack.push(current.clone());
                current.move_left();
            } else if current.right().is_some() {
                self.right_stack.push(current.clone());
                current.move_right();
            } else {
                self.right_stack.push(current.clone());
                break;
            }
        }
    }

    pub fn new(mut root: Cursor) -> Self {
        let mut iter = Self {
            left_stack: Vec::new(),
            right_stack: Vec::new(),
            marker: PhantomData::default(),
        };
        iter.push_deep_most_chain(&mut root);
        iter
    }
}

impl<'a, T: 'a, Tree, Cursor: BinTreeNode<'a, Tree, Elem = T> + Clone> Iterator
    for PostOrderIter<'a, Cursor, Tree>
{
    type Item = &'a Cursor::Elem;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.right_stack.pop() {
            current.into_ref()
        } else if let Some(mut current) = self.left_stack.pop() {
            self.right_stack.push(current.clone());
            current.move_right();
            self.push_deep_most_chain(&mut current);
            self.right_stack.pop().unwrap().into_ref()
        } else {
            None
        }
    }
}
