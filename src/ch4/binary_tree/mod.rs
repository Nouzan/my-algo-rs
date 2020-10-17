pub mod coding_tree;
pub mod cursor;
pub mod cursor_ext;
pub mod iter;
pub mod linked_binary_tree;
pub mod vec_binary_tree;

pub use cursor::*;
pub use cursor_ext::*;

/// 不可变二叉树特质.
pub trait BinTree {
    /// 内容类型.
    type Elem;

    /// 不可变游标类型.
    type Cursor<'a, T: 'a>: BinTreeCursor<'a, Elem = T> + Clone;

    /// 是否为空树.
    fn is_empty<'a>(&'a self) -> bool
    where
        Self: Sized,
    {
        self.cursor().is_empty_subtree()
    }

    /// 创建一个只读结点游标.
    fn cursor<'a>(&'a self) -> Self::Cursor<'a, Self::Elem>;
}

/// 可变二叉树特质.
pub trait BinTreeMut: BinTree {
    /// 可变游标类型.
    type CursorMut<'a>: BinTreeCursorMut<'a, Elem = Self::Elem, SubTree = Self>;

    /// 创建一个可变结点游标.
    fn cursor_mut<'a>(&'a mut self) -> Self::CursorMut<'a>;
}
