pub mod iter;
pub mod node;
pub mod node_ext;
pub mod vec_binary_tree;

pub use node::*;
pub use node_ext::*;

/// 不可变二叉树特质.
pub trait BinTree {
    /// 内容类型.
    type Elem;

    /// 不可变结点类型.
    type Node<'a, T: 'a>: BinTreeNode<'a, Elem = T> + Clone;

    /// 是否为空树.
    fn is_empty(&self) -> bool {
        self.cursor().is_empty_subtree()
    }

    /// 创建一个只读结点游标.
    fn cursor(&self) -> Self::Node<'_, Self::Elem>;
}

/// 可变二叉树特质.
pub trait BinTreeMut<C>: BinTree {
    /// 创建一个可变结点游标.
    fn cursor_mut<'a>(&'a mut self) -> C
    where
        C: BinTreeNodeMut<'a, Tree = Self>,
    {
        C::cursor_mut(self)
    }
}
