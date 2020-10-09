pub mod vec_binary_tree;

/// 不可变二叉树结点特质.
pub trait BinTreeNode: BinTree {
    /// 若为空树则返回`None`，否则返回当前结点(根)的内容的引用.
    fn as_ref(&self) -> Option<&Self::Elem>;

    /// 若为空树或不含左孩子则返回`None`，否则返回左孩子的内容的引用.
    fn left(&self) -> Option<&Self::Elem>;

    /// 若为空树或不含右孩子则返回`None`，否则返回右孩子的内容的引用.
    fn right(&self) -> Option<&Self::Elem>;

    /// 若为空树则`no-op`，否则变为左子树.
    fn move_left(&mut self);

    /// 若为空树则`no-op`，否则变为右子树.
    fn move_right(&mut self);
}

/// 可变二叉树结点特质.
pub trait BinTreeNodeMut: BinTreeNode + BinTreeMut {
    type Tree: BinTree;

    /// 若为空树则返回`None`，否则返回当前结点(根)的内容的可变引用.
    fn as_mut(&mut self) -> Option<&mut Self::Elem>;

    /// 若为空树或不含左孩子则返回`None`，否则返回左孩子的内容的可变引用.
    fn left_mut(&mut self) -> Option<&mut Self::Elem>;

    /// 若为空树或不含右孩子则返回`None`，否则返回右孩子的内容的可变引用.
    fn right_mut(&mut self) -> Option<&mut Self::Elem>;

    /// 插入一个元素作为根.
    /// 若不为空树，则是`no-op`并返回被插入的元素，
    /// 否则将元素作为根插入树中，并返回`None`.
    fn insert_as_root(&mut self, elem: Self::Elem) -> Option<Self::Elem>;

    /// 插入一个元素作为左孩子.
    /// - 若为空树或左孩子不为空则为`no-op`，并返回被插入的元素.
    /// - 否则元素将作为左孩子插入树中，并返回`None`.
    fn insert_as_left(&mut self, elem: Self::Elem) -> Option<Self::Elem>;

    /// 插入一个元素作为右孩子.
    /// - 若为空树或右孩子不为空则为`no-op`，并返回被插入的元素.
    /// - 否则元素将作为右孩子插入树中，并返回`None`.
    fn insert_as_right(&mut self, elem: Self::Elem) -> Option<Self::Elem>;

    /// 摘取左子树并返回. 若树为空，则返回`None`，若子树为空，则返回空树.
    fn take_left(&mut self) -> Option<Self::Tree>
    where
        Self::Tree: Sized;

    /// 摘取右子树并返回. 若树为空，则返回`None`，若子树为空，则返回空树.
    fn take_right(&mut self) -> Option<Self::Tree>
    where
        Self::Tree: Sized;

    /// 把一棵树作为左子树接入. 操作后`other`变为空树.
    /// # Panics
    /// 若左子树不为空则报错.
    fn append_left(&mut self, other: &mut Self);

    /// 把一棵树作为右子树接入. 操作后`other`变为空树.
    /// # Panics
    /// 若右子树不为空则报错.
    fn append_right(&mut self, other: &mut Self);

    /// 消耗整棵树返回根的内容. 若为空树，则返回`None`.
    fn into_inner(self) -> Option<Self::Elem>;
}

/// 不可变二叉树特质.
pub trait BinTree {
    /// 内容类型.
    type Elem;

    /// 不可变结点类型
    type Node<'a, T: 'a>: BinTreeNode<Elem = T> + Clone;

    /// 是否为空树.
    fn is_empty(&self) -> bool {
        self.cursor().as_ref().is_none()
    }

    /// 创建一个只读结点游标.
    fn cursor<'a>(&'a self) -> Self::Node<'a, Self::Elem>;

    /// 层序遍历迭代器.
    fn in_order_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self::Elem> + 'a> {
        let mut queue = VecDeque::new();
        if self.cursor().as_ref().is_some() {
            queue.push_back(self.cursor())
        }
        Box::new(InOrderIter {
            queue,
            marker: PhantomData::default(),
        })
    }
}

/// 可变二叉树特质.
pub trait BinTreeMut: BinTree {
    /// 可变结点类型
    type NodeMut<'a, T: 'a>: BinTreeNodeMut<Elem = T>;

    /// 创建一个可变结点游标.
    fn cursor_mut<'a>(&'a mut self) -> Self::NodeMut<'a, Self::Elem>;
}

use std::collections::VecDeque;
use std::marker::PhantomData;

pub struct InOrderIter<'a, T> {
    queue: VecDeque<T>,
    marker: PhantomData<&'a T>,
}

impl<'a, T: BinTreeNode + Clone> Iterator for InOrderIter<'a, T>
where
    T::Elem: 'a,
{
    type Item = &'a T::Elem;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cursor) = self.queue.pop_front() {
            let mut left = cursor.clone();
            let mut right = cursor.clone();
            left.move_left();
            right.move_right();
            if left.as_ref().is_some() {
                self.queue.push_back(left);
            }
            if right.as_ref().is_some() {
                self.queue.push_back(right);
            }
            unsafe {
                let ptr: *const _ = cursor.as_ref().unwrap();
                Some(&*ptr)
            }
        } else {
            None
        }
    }
}
