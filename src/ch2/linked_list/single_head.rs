type Link<T> = Option<Box<Node<T>>>;

/// 普通结点.
pub struct ItemNode<T> {
    elem: T,
    next: Link<T>,
}

/// 单链表结点.
pub enum Node<T> {
    Item(ItemNode<T>),
    Head(Link<T>),
}

impl<T> Node<T> {
    /// 创建一个新的普通结点.
    fn new(elem: T) -> Box<Self> {
        Box::new(Node::Item(ItemNode { elem, next: None }))
    }

    /// 转换为一个`ItemNode`的可变引用.
    /// # Panics
    /// 如果对`Head`进行转换则报错.
    fn as_item_node_mut_unchecked(&mut self) -> &mut ItemNode<T> {
        match self {
            Self::Head(_) => panic!("A `Head` cannot be make into an `ItemNode`."),
            Self::Item(node) => node,
        }
    }

    /// 分割引用.
    /// # Panics
    /// 如果对`Head`进行引用分割则报错.
    fn split_mut_unchecked(&mut self) -> (Option<&mut Node<T>>, &mut T) {
        match self {
            Self::Head(_) => panic!("Trying to get value from `Head`."),
            Self::Item(node) => (node.next.as_deref_mut(), &mut node.elem),
        }
    }

    /// 返回当前结点后继的只读引用, 若后继为`None`则返回`None`.
    fn next(&self) -> Option<&Node<T>> {
        match self {
            Self::Head(next) => next.as_ref().map(|node| &**node),
            Self::Item(node) => node.next.as_deref(),
        }
    }

    /// 返回当前结点后继的可变引用, 若后继为`None`则返回`None`.
    fn next_mut(&mut self) -> Option<&mut Node<T>> {
        match self {
            Self::Head(next) => next.as_mut().map(|node| &mut **node),
            Self::Item(node) => node.next.as_deref_mut(),
        }
    }

    /// 返回当前结点内容的可变引用.
    /// # Panics
    /// 方法假设当前结点为普通结点(`ItemNode`), 若不是普通结点, 则报错.
    fn elem_mut_unchecked(&mut self) -> &mut T {
        let (_, elem) = self.split_mut_unchecked();
        elem
    }

    /// 返回当前结点内容的只读引用.
    /// # Panics
    /// 方法假设当前结点为普通结点(`ItemNode`), 若不是普通结点, 则报错.
    fn elem_unchecked(&self) -> &T {
        match self {
            Self::Head(_) => panic!("Trying to get value from `Head`."),
            Self::Item(node) => &node.elem,
        }
    }

    /// 将当前结点的后继替换为`next`, 并返回原来的后继.
    fn link(&mut self, next: Link<T>) -> Link<T> {
        match self {
            Self::Head(link) => {
                let old = link.take();
                *link = next;
                old
            }
            Self::Item(node) => {
                let old = node.next.take();
                node.next = next;
                old
            }
        }
    }
}

/// 带头结点的单链表.
/// # Correctness
/// 该单链表的实现必须保证任何时候任何结点的后继都是`Option<ItemNode>`.
pub struct LinkedList<T> {
    head: Box<Node<T>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self {
            head: Box::new(Node::Head(None)),
        }
    }
}

/// 单链表迭代器.
pub struct IterMut<'a, T: 'a> {
    inner: Option<&'a mut ItemNode<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.take().map(|node| {
            let (next, elem) = (
                node.next
                    .as_mut()
                    .map(|node| node.as_item_node_mut_unchecked()),
                &mut node.elem,
            );
            self.inner = next;
            elem
        })
    }
}

/// 单链表游标.
/// 该结构保存着的是“当前”结点的直接前驱(`prev`).
/// # Correctness
/// 必须保证`prev`结点的所有后继都是`Option<ItemNode>`.
pub struct CursorMut<'a, T: 'a> {
    prev: Option<&'a mut Node<T>>, // 始终保存着“当前”结点的前驱, 因此“当前”结点必然为`Option<ItemNode>`.
}

impl<'a, T> CursorMut<'a, T> {
    /// 移动游标到当前结点的直接后继.
    /// 逻辑上, 游标可以指向`None`的后继(即`prev`为`None`), 但这并没有实际意义.
    pub fn move_next(&mut self) {
        // # Correctness
        // 如果`prev`的所有后继都是`Option<ItemNode>`,
        // 那么`prev`的后继的所有后继自然也是`Option<ItemNode>`.
        // 故依然保持`Correctness`假设.
        //
        // 反复调用该方法最终必然会使得`prev`为`None`,
        // 而当`prev`为`None`时, 该方法是`no-op`.
        if let Some(prev) = self.prev.take() {
            self.prev = prev.next_mut();
        }
    }

    /// 获取当前结点内容的只读引用.
    /// 如果游标的当前结点为链表末尾的`None`或`None`的后继, 则返回`None`.
    pub fn peek(&self) -> Option<&T> {
        if let Some(prev) = self.prev.as_ref() {
            prev.next().as_ref().map(|node| {
                node.elem_unchecked() // `node`是后继, 因此必然是`ItemNode`.
            })
        } else {
            None
        }
    }

    /// 获取当前结点内容的可变引用.
    /// 如果游标的当前结点为链表末尾的`None`或`None`的后继, 则返回`None`.
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if let Some(prev) = self.prev.as_mut() {
            prev.next_mut().map(|node| {
                node.elem_mut_unchecked() // `node`是后继, 因此必然是`ItemNode`.
            })
        } else {
            None
        }
    }

    /// 移除当前结点并返回, 当前结点将会变为原来结点的后继.
    /// - 如果游标的当前结点为尾结点, 那么移除后, 当前结点变为`None`;
    /// - 如果游标的当前结点为`None`或`None`的后继, 则该方法是`no-op`, 返回值为`None`.
    pub fn remove_current(&mut self) -> Link<T> {
        // # Correctness
        // 如果`prev`的所有后继都是`Option<ItemNode>`,
        // 那么`prev`的后继的所有后继自然也是`Option<ItemNode>`.
        // 故依然保持`Correctness`假设.
        if let Some(prev) = self.prev.as_mut() {
            let mut current = prev.link(None);
            if let Some(node) = &mut current {
                let next = node.link(None);
                prev.link(next);
            }
            current
        } else {
            None
        }
    }
}

impl<T> LinkedList<T> {
    /// 返回一个从首结点开始的可变迭代器.
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            inner: self
                .head
                .next_mut()
                .map(|node| node.as_item_node_mut_unchecked()),
        }
    }

    /// 返回一个当前结点为首结点的可变游标.
    pub fn cursor_mut(&mut self) -> CursorMut<T> {
        CursorMut {
            prev: Some(self.head.as_mut()),
        }
    }
}

use std::cmp::PartialEq;

impl<T: PartialEq> LinkedList<T> {
    /// 在首部插入新元素.
    pub fn push_front(&mut self, elem: T) {
        let first = self.head.link(Some(Node::new(elem)));
        self.head.next_mut().unwrap().link(first);
    }

    /// 删除所有值等于`x`的元素.
    pub fn delete_all(&mut self, x: &T) {
        let mut cursor = self.cursor_mut();
        while let Some(current) = cursor.peek_mut() {
            if *current == *x {
                cursor.remove_current();
            }
            cursor.move_next();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_basic(data: Vec<isize>) {
            let mut list = LinkedList::default();
            for v in data.iter().rev() {
                list.push_front(*v);
            }
            for (i, v) in list.iter_mut().enumerate() {
                prop_assert_eq!(data[i], *v);
            }
        }
    }

    proptest! {
        #[test]
        fn test_delete_all(data: Vec<isize>) {
            let mut list = LinkedList::default();
            for v in data.iter().rev() {
                list.push_front(*v);
            }
            if !data.is_empty() {
                let target = data[0];
                list.delete_all(&target);
                let mut iter = list.iter_mut();
                for v in data {
                    if v != target {
                        assert_eq!(Some(v), iter.next().copied())
                    }
                }
            } else {
                list.delete_all(&1);
            }
        }
    }
}
