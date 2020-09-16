use super::*;
use crate::ch2::linked_list::{LinearCursor, LinearCursorMut};

// 关于游标, 我们引入一个新的不变式:
// - `current`(若有)是链表所有权内的合法指针.
// - `index`(若表不为空)表示从首结点沿后继链的距离(特别地, 首结点的`index`为0, 幽灵结点的`index`为`len`).

/// 只读游标.
/// 指向双循环链表的一个结点.
/// 特别地, 我们形式地插入一个幽灵结点, 作为首结点的前驱、尾结点的后继.
/// 幽灵结点的意义是标记一轮遍历的完成.
/// `Cursor`的生命期与它所对应的链表只读引用的生命期是一致的, 这保证了共享只读引用的合法性.
pub struct Cursor<'a, T: 'a> {
    index: usize,

    /// 表空或指向幽灵结点时为`None`.
    current: Option<Link<T>>,
    list: &'a LinkedList<T>,
}

impl<'a, T: 'a> Cursor<'a, T> {
    /// 使用一个链表的引用构造一个新的`Cursor`.
    pub fn new(list: &'a LinkedList<T>) -> Self {
        Self {
            index: 0,
            current: list.head,
            list,
        }
    }

    /// `Cursor`向左移动, 指向它的前驱.
    pub fn move_prev(&mut self) {
        // 根据不变式, `node`是合法的.
        // 而根据链表的不变式, `prev`也是合法的, 这保持了游标的不变式.
        unsafe {
            if let Some(node) = self.current.take() {
                if node != self.list.head.unwrap() {
                    self.index -= 1;
                    self.current = Some(node.as_ref().prev);
                } else {
                    self.index = self.list.len();
                }
            } else if let Some(head) = self.list.head {
                self.index = self.list.len() - 1;
                self.current = Some(head.as_ref().prev);
            }
        }
    }
}

impl<'a, T: 'a> LinearCursor<'a, T> for Cursor<'a, T> {
    fn move_next(&mut self) {
        // 根据不变式, `node`是合法的.
        // 而根据链表的不变式, `next`也是合法的, 这保持了游标的不变式.
        unsafe {
            if let Some(node) = self.current.take() {
                let tail = self.list.head.unwrap().as_ref().prev;
                if node != tail {
                    self.index += 1;
                    self.current = Some(node.as_ref().next);
                } else {
                    self.index = self.list.len();
                }
            } else {
                self.index = 0;
                self.current = self.list.head;
            }
        }
    }

    fn peek(&self) -> Option<&T> {
        // 根据不变式, `node`是合法的, 且生命期限制保证了共享只读引用的合法性.
        self.current.map(|node| unsafe { &(*node.as_ptr()).elem })
    }

    fn is_front_or_empty(&self) -> bool {
        self.current == self.list.head
    }

    fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    fn is_ghost(&self) -> bool {
        !self.list.is_empty() && self.current.is_none()
    }

    fn index(&self) -> Option<usize> {
        if self.current.is_some() {
            Some(self.index)
        } else {
            None
        }
    }

    fn into_ref(self) -> Option<&'a T> {
        self.current.map(|node| unsafe { &(*node.as_ptr()).elem })
    }
}

/// 可变游标.
/// 指向双循环链表的一个结点.
/// 特别地, 我们形式地插入一个幽灵结点, 作为首结点的前驱、尾结点的后继.
/// 幽灵结点的意义是标记一轮遍历的完成.
///
/// 可变游标的生命期与它对应的链变的可变引用的生命期是一致的, 这保证了可变引用的合法性.
pub struct CursorMut<'a, T: 'a> {
    index: usize,

    /// 表空或指向幽灵结点时为`None`.
    current: Option<Link<T>>,
    list: &'a mut LinkedList<T>,
}

impl<'a, T: 'a> CursorMut<'a, T> {
    /// 将新结点作为当前结点的后继插入.
    /// 若当前结点为幽灵结点或表空, 则作为首结点插入, 插入后前种情况仍指向幽灵结点, 后种情况则指向新的首结点.
    fn insert_after_node(&mut self, node: Box<Node<T>>) {
        if self.current.is_none() {
            self.list.push_front_node(node);
            self.index = if self.list.len() == 1 {
                0
            } else {
                self.list.len()
            };
            self.current = if self.list.len() == 1 {
                self.list.head
            } else {
                None
            };
        } else {
            let node: Link<T> = Box::leak(node).into();
            // 此时表不为空, 则根据游标不变式和链表不变式, 下面的操作都是安全的.
            // 操作完后, `node`共享了它的两个指针给当前结点(作为后继)和当前结点的后继(作为前驱),
            // `current`及其后继的指针并未丢失和新增, 因此依然保持链表不变式.
            unsafe {
                let current = self.current.unwrap();
                let next = current.as_ref().next;
                (*node.as_ptr()).next = next;
                (*node.as_ptr()).prev = current;
                (*current.as_ptr()).next = node;
                (*next.as_ptr()).prev = node;
            }
            // 修正链表长度, 游标下标根据定义不变.
            self.list.len += 1;
        }
    }

    /// 将新结点作为当前结点的前驱插入.
    /// 若表空则作为首结点插入, 并指向首结点.
    /// 若为幽灵结点, 则作为首结点的前驱插入, 但并不改变首结点.
    fn insert_before_node(&mut self, node: Box<Node<T>>) {
        if self.is_front_or_empty() {
            self.list.push_front_node(node);
            if self.current.is_none() {
                self.index = 0;
                self.current = self.list.head;
            } else {
                self.index = 1;
            }
        } else {
            let node: Link<T> = Box::leak(node).into();
            // 此时表不为空, 则根据游标不变式和链表不变式, 下面的操作都是安全的.
            // 操作完后, `node`共享了它的两个指针给当前结点(作为前驱)和当前结点的前驱(作为后继),
            // `current`及其前驱的指针并未丢失和新增, 因此依然保持链表不变式.
            unsafe {
                // 若当前结点为幽灵结点, 则用首结点替代.
                let current = self.current.unwrap_or_else(|| self.list.head.unwrap());
                let prev = current.as_ref().prev;
                (*node.as_ptr()).next = current;
                (*node.as_ptr()).prev = prev;
                (*prev.as_ptr()).next = node;
                (*current.as_ptr()).prev = node;
            }
            // 修正链表长度与游标下标.
            self.index += 1;
            self.list.len += 1;
        }
    }

    fn remove_current_node(&mut self) -> Option<Box<Node<T>>> {
        self.current.take().map(|current| unsafe {
            // 此时表不为空. 根据游标和链表不变式, 下面的操作都是安全的, 指针都是合法的.
            let prev = current.as_ref().prev;
            let next = current.as_ref().next;
            // 这里保持了链表不变式.
            (*prev.as_ptr()).next = next;
            (*next.as_ptr()).prev = prev;
            if next == current {
                // 根据循环链表性质, 说明这是表的唯一结点.
                // 操作后表空, 这里保持了游标和链表的不变式.
                self.list.head = None;
                self.current = None;
            } else if self.list.head.unwrap() == current {
                // 要删除的结点是首结点且并非唯一结点(`next` != `current`).
                // 这里保持了游标不变式, 并保持了首结点的正确性.
                self.list.head = Some(next);
                self.current = Some(next);
            } else {
                // 要删除的结点不是首结点.
                // 这里保持了游标不变式.
                self.current = Some(next);
            }
            // 修正链表长度与游标下标.
            self.list.len -= 1;
            if !self.list.is_empty() {
                self.index %= self.list.len();
            }
            Box::from_raw(current.as_ptr())
        })
    }
}

impl<'a, T: 'a> CursorMut<'a, T> {
    /// 使用链表的可变引用创建一个新的`CursorMut`.
    pub fn new(list: &'a mut LinkedList<T>) -> Self {
        Self {
            index: 0,
            current: list.head,
            list,
        }
    }

    /// `Cursor`向左移动, 指向它的前驱.
    pub fn move_prev(&mut self) {
        // 根据不变式, `node`是合法的.
        // 而根据链表的不变式, `prev`也是合法的, 这保持了游标的不变式.
        unsafe {
            if let Some(node) = self.current.take() {
                if node != self.list.head.unwrap() {
                    self.index -= 1;
                    self.current = Some(node.as_ref().prev);
                } else {
                    self.index = self.list.len();
                }
            } else if let Some(head) = self.list.head {
                self.index = self.list.len() - 1;
                self.current = Some(head.as_ref().prev);
            }
        }
    }
}

impl<'a, T: 'a> LinearCursor<'a, T> for CursorMut<'a, T> {
    fn move_next(&mut self) {
        // 根据不变式, `node`是合法的.
        // 而根据链表的不变式, `next`也是合法的, 这保持了游标的不变式.
        unsafe {
            if let Some(node) = self.current.take() {
                let tail = self.list.head.unwrap().as_ref().prev;
                if node != tail {
                    self.index += 1;
                    self.current = Some(node.as_ref().next);
                } else {
                    self.index = self.list.len();
                }
            } else {
                self.index = 0;
                self.current = self.list.head;
            }
        }
    }

    fn peek(&self) -> Option<&T> {
        // 根据不变式, `node`是合法的.
        // 这个操作将会冻结可变游标.
        // 因为返回的只读引用的生命期和可变游标的只读引用的生命期一样长.
        self.current.map(|node| unsafe { &(*node.as_ptr()).elem })
    }

    fn is_front_or_empty(&self) -> bool {
        self.as_cursor().is_front_or_empty()
    }

    fn is_empty(&self) -> bool {
        self.as_cursor().is_empty()
    }

    fn is_ghost(&self) -> bool {
        self.as_cursor().is_ghost()
    }

    fn index(&self) -> Option<usize> {
        if self.current.is_some() {
            Some(self.index)
        } else {
            None
        }
    }

    fn into_ref(self) -> Option<&'a T> {
        self.current.map(|node| unsafe { &(*node.as_ptr()).elem })
    }
}

impl<'a, T: 'a> LinearCursorMut<'a, T> for CursorMut<'a, T> {
    type Cursor<'b, U: 'b> = Cursor<'b, U>;

    /// 转换为一个只读游标.
    ///
    /// 这个操作将会冻结可变游标.
    /// 因为新产生的`Cursor`的生命期与可变游标的只读引用的生命期一样长,
    /// 所以当我们再一次拿到可变游标的可变引用时, 该`Cursor`将会不可用.
    fn as_cursor(&self) -> Self::Cursor<'_, T> {
        Cursor {
            index: self.index,
            current: self.current,
            list: self.list,
        }
    }

    /// 获取所指结点内容的可变引用. 若表空则返回`None`.
    /// 返回的可变引用的生命期受限于可变游标的可变引用的生命期, 因此受限于链表的生命期.
    fn peek_mut(&mut self) -> Option<&mut T> {
        self.current
            .map(|node| unsafe { &mut (*node.as_ptr()).elem })
    }

    /// 在当前结点前插入新值, 游标所指结点不变. 但注意以下行为:
    /// - 若表空, 则新值将作为首结点插入, 游标指向首结点.
    /// - 若所指结点为首结点, 则新值将作为尾结点插入(不改变头指针).
    fn insert_before(&mut self, elem: T) -> Option<T> {
        self.insert_before_node(Box::new(Node::new(elem)));
        None
    }

    /// 在当前结点后插入新值, 游标所指结点不变.
    /// 若表空, 则新值将作为首结点插入, 游标指向首结点.
    fn insert_after(&mut self, elem: T) -> Option<T> {
        self.insert_after_node(Box::new(Node::new(elem)));
        None
    }

    /// 在当前结点前插入新值, 游标指向新插入的结点, 插入成功时返回`None`.
    /// 若位置不合法, 则返回被插入的值.
    fn insert_before_as_current(&mut self, elem: T) -> Option<T> {
        self.insert_before(elem);
        self.move_prev();
        if self.is_ghost() {
            self.move_prev();
        }
        None
    }

    /// 在当前结点后插入新值, 游标指向新插入的结点, 插入成功时返回`None`.
    /// 若位置不合法, 则返回被插入的值.
    fn insert_after_as_current(&mut self, elem: T) -> Option<T> {
        self.insert_after(elem);
        self.move_next();
        if self.is_ghost() {
            self.move_next();
        }
        None
    }

    /// 删除当前所指结点并返回其内容, 游标改为指向它的后继. 若表空则返回`None`.
    /// - 如果所指结点是首结点, 则头指针将会改为指向它的后继.
    /// - 如果所指结点是链表中唯一结点, 则操作完后表空且游标指向`None`.
    fn remove_current(&mut self) -> Option<T> {
        self.remove_current_node().map(|node| node.into_elem())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lifetime() {
        let mut list = LinkedList::from(vec![1, 2, 3, 4, 5]);
        let mut cm = list.cursor_front_mut();
        cm.move_next();
        let mut c1 = cm.as_cursor();
        let mut c2 = cm.as_cursor();
        c1.move_next();
        c2.move_next();
        assert_eq!(c1.peek(), c2.peek());
        let idx = c1.index();
        cm.move_next();
        assert_eq!(idx, cm.index());
    }

    #[test]
    fn test_lifetime_2() {
        let mut list = LinkedList::from(vec![1, 2, 3, 4, 5]);
        let mut cm = list.cursor_front_mut();
        cm.move_next();
        let mut c1 = list.cursor_front();
        let mut c2 = list.cursor_front();
        c1.move_next();
        c2.move_next();
        let mut cm = list.cursor_front_mut();
        cm.move_next();
    }
}
