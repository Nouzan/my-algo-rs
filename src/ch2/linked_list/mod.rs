pub mod cdll;
pub mod shll;
pub mod single;
pub mod singly;

pub use singly::*;

/// 只读线性游标接口, 用于实现只读的循位置访问.
pub trait LinearCursor<T> {
    /// 游标向右移动, 指向它的后继.
    fn move_next(&mut self);

    /// 获得所指结点的元素的只读引用.
    fn peek(&self) -> Option<&T>;

    /// 表空或所指结点为首结点时返回`true`.
    /// 特别地, 指向“幽灵”结点时返回`false`.
    fn is_front_or_empty(&self) -> bool;

    /// 表为空时返回`true`.
    /// 特别地, 若指向“幽灵”结点则返回`false`.
    fn is_empty(&self) -> bool;

    /// 是否为“幽灵”结点.
    fn is_ghost(&self) -> bool;

    /// 所指结点相对与首结点的偏移. 若表空则返回`None`.
    fn index(&self) -> Option<usize>;
}

/// 可变游标接口, 用于实现可变的循位置访问.
pub trait LinearCursorMut<T>: LinearCursor<T> {
    type Cursor<'a, U: 'a>: LinearCursor<U>;

    /// 转换为一个只读游标.
    fn as_cursor(&self) -> Self::Cursor<'_, T>;

    /// 获取所指结点内容的可变引用. 若表空则返回`None`.
    fn peek_mut(&mut self) -> Option<&mut T>;

    /// 在当前结点前插入新值, 游标所指结点不变, 插入成功时返回`None`.
    /// 若表空, 则新值将作为首结点插入, 游标指向首结点.
    /// 若位置不合法, 则返回被插入的值.
    fn insert_before(&mut self, elem: T) -> Option<T>;

    /// 在当前结点后插入新值, 游标所指结点不变, 插入成功时返回`None`.
    /// 若表空, 则新值将作为首结点插入, 游标指向首结点.
    /// 若位置不合法, 则返回被插入的值.
    fn insert_after(&mut self, elem: T) -> Option<T>;

    /// 在当前结点前插入新值, 游标指向新插入的结点, 插入成功时返回`None`.
    /// 若位置不合法, 则返回被插入的值.
    fn insert_before_as_current(&mut self, elem: T) -> Option<T>;

    /// 在当前结点后插入新值, 游标指向新插入的结点, 插入成功时返回`None`.
    /// 若位置不合法, 则返回被插入的值.
    fn insert_after_as_current(&mut self, elem: T) -> Option<T>;

    /// 删除当前所指结点并返回其内容, 游标改为指向它的后继. 若表空则返回`None`.
    fn remove_current(&mut self) -> Option<T>;
}

/// 单链表接口.
pub trait SinglyLinkedList<T>: Default {
    type Cursor<'a, U: 'a>: LinearCursor<U>;
    type CursorMut<'a, U: 'a>: LinearCursorMut<U>;

    /// 链表是否为空.
    fn is_empty(&self) -> bool;

    /// 获取表的长度.
    fn len(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            let mut cursor = self.cursor_front();
            let mut len = 1;
            cursor.move_next();
            while !(cursor.is_front_or_empty() || cursor.is_ghost()) {
                len += 1;
                cursor.move_next();
            }
            len
        }
    }

    /// 把新值作为新的首结点插入.
    fn push_front(&mut self, elem: T);

    /// 弹出首结点, 首结点的直接后继(若有)将成为新的首结点.
    fn pop_front(&mut self) -> Option<T>;

    /// 冻结链表, 创建指向首结点(若有)的只读游标.
    fn cursor_front(&self) -> Self::Cursor<'_, T>;

    /// 创建指向首结点(若有)的可变游标.
    fn cursor_front_mut(&mut self) -> Self::CursorMut<'_, T>;
}
