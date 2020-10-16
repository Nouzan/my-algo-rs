pub mod coding_tree;
pub mod iter;
pub mod linked_binary_tree;
pub mod node;
pub mod node_ext;
pub mod vec_binary_tree;

pub use node::*;
pub use node_ext::*;

/// 不可变二叉树特质.
pub trait BinTree<Cursor> {
    /// 内容类型.
    type Elem;

    // /// 不可变结点类型.
    // type Node<'a, T: 'a>: BinTreeNode<'a, Elem = T> + Clone;

    /// 是否为空树.
    fn is_empty<'a>(&'a self) -> bool
    where
        Cursor: BinTreeNode<'a, Self>,
        Self: Sized,
    {
        self.cursor().is_empty_subtree()
    }

    /// 创建一个只读结点游标.
    fn cursor<'a>(&'a self) -> Cursor
    where
        Cursor: BinTreeNode<'a, Self>,
        Self: Sized,
    {
        Cursor::new(self)
    }
}

/// 可变二叉树特质.
pub trait BinTreeMut<Cursor, CursorMut>: BinTree<Cursor> {
    /// 创建一个可变结点游标.
    fn cursor_mut<'a>(&'a mut self) -> CursorMut
    where
        CursorMut: BinTreeNodeMut<'a, Self>,
        Self: Sized,
    {
        CursorMut::new(self)
    }
}
