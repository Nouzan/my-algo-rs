use super::iter::{InOrderIter, MidOrderIter, PostOrderIter, PreOrderIter};
use super::node::{BinTreeNode, BinTreeNodeMut};
use super::BinTreeMut;

impl<'a, T: BinTreeNode<'a>> BinTreeNodeExt<'a> for T {}

pub trait BinTreeNodeExt<'a>: BinTreeNode<'a> {
    /// 创建一个层序遍历迭代器.
    fn in_order_iter(&self) -> InOrderIter<Self>
    where
        Self: Sized + Clone,
    {
        let root = if !self.is_empty_subtree() {
            Some(self.clone())
        } else {
            None
        };
        InOrderIter::new(root)
    }

    /// 创建一个前序遍历迭代器.
    fn pre_order_iter(&self) -> PreOrderIter<Self>
    where
        Self: Sized + Clone,
    {
        let root = if !self.is_empty_subtree() {
            Some(self.clone())
        } else {
            None
        };
        PreOrderIter::new(root)
    }

    /// 创建一个中序遍历迭代器.
    fn mid_order_iter(&'a self) -> MidOrderIter<Self>
    where
        Self: Sized + Clone,
    {
        MidOrderIter::new(self.clone())
    }

    /// 创建一个后序遍历迭代器.
    fn post_order_iter(&'a self) -> PostOrderIter<Self>
    where
        Self: Sized + Clone,
    {
        PostOrderIter::new(self.clone())
    }
}

impl<'a, E, C: BinTreeNodeMut<'a, Elem = E> + BinTreeMut<Elem = E>> FrozenNodeMutExt<'a, E> for C {}

fn pre_order_for_each_inner<E, C1, F>(cursor: &mut C1, f: &mut F)
where
    F: FnMut(&mut E),
    C1: for<'b> BinTreeNodeMut<'b, Elem = E> + BinTreeMut<Elem = E, NodeMut = C1>,
{
    if let Some(elem) = cursor.as_mut() {
        f(elem);
        let mut left = cursor.cursor_mut();
        left.move_left();
        pre_order_for_each_inner(&mut left, f);
        let mut right = cursor.cursor_mut();
        right.move_right();
        pre_order_for_each_inner(&mut right, f);
    }
}

pub trait FrozenNodeMutExt<'a, T>: BinTreeNodeMut<'a, Elem = T> + BinTreeMut<Elem = T> {
    fn pre_order_for_each<F, C1>(&mut self, mut f: F)
    where
        F: FnMut(&mut T),
        Self: BinTreeMut<NodeMut = C1>,
        C1: for<'b> BinTreeNodeMut<'b, Elem = T> + BinTreeMut<Elem = T, NodeMut = C1>,
    {
        pre_order_for_each_inner(&mut self.cursor_mut(), &mut f);
    }
}
