use super::cursor::BinTreeCursor;
use super::iter::{InOrderIter, MidOrderIter, PostOrderIter, PreOrderIter};

impl<'a, T: BinTreeCursor<'a>> BinTreeCursorExt<'a> for T {}

pub trait BinTreeCursorExt<'a>: BinTreeCursor<'a> {
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

    /// 转变为一个中序遍历迭代器.
    fn into_mid_order_iter(self) -> MidOrderIter<'a, Self>
    where
        Self: Sized + Clone,
    {
        MidOrderIter::new(self)
    }

    /// 创建一个后序遍历迭代器.
    fn post_order_iter(&'a self) -> PostOrderIter<Self>
    where
        Self: Sized + Clone,
    {
        PostOrderIter::new(self.clone())
    }
}
