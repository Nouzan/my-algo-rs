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

    // /// 分割引用.
    // /// # Panics
    // /// 如果对`Head`进行引用分割则报错.
    // fn split_mut_unchecked(&mut self) -> (Option<&mut Node<T>>, &mut T) {
    //     match self {
    //         Self::Head(_) => panic!("Trying to get value from `Head`."),
    //         Self::Item(node) => (node.next.as_deref_mut(), &mut node.elem),
    //     }
    // }

    fn next_mut(&mut self) -> Option<&mut Node<T>> {
        match self {
            Self::Head(next) => next.as_mut().map(|node| &mut **node),
            Self::Item(node) => node.next.as_deref_mut(),
        }
    }

    // fn elem_mut_unchecked(&mut self) -> &mut T {
    //     let (_, elem) = self.split_mut_unchecked();
    //     elem
    // }

    fn elem_unchecked(&self) -> &T {
        match self {
            Self::Head(_) => panic!("Trying to get value from `Head`."),
            Self::Item(node) => &node.elem,
        }
    }

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

/// 单链表游标.
pub struct CursorMut<'a, T: 'a> {
    inner: Option<&'a mut ItemNode<T>>,
}

impl<T> LinkedList<T> {
    pub fn cursor_mut(&mut self) -> CursorMut<T> {
        CursorMut {
            inner: self
                .head
                .next_mut()
                .map(|node| node.as_item_node_mut_unchecked()),
        }
    }
}

impl<'a, T> Iterator for CursorMut<'a, T> {
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

use std::cmp::PartialEq;

impl<T: PartialEq> LinkedList<T> {
    /// 在首部插入新元素.
    pub fn push_front(&mut self, elem: T) {
        let first = self.head.link(Some(Node::new(elem)));
        self.head.next_mut().unwrap().link(first);
    }

    /// 删除所有值等于`x`的元素.
    pub fn delete_all(&mut self, x: &T) {
        let mut prev = self.head.as_mut();
        while let Some(node) = prev.next_mut() {
            let elem = node.elem_unchecked(); // `head`的后继必然为`ItemNode`, `ItemNode`的后继也必然为`ItemNode`.
            if *elem == *x {
                let next = node.link(None);
                prev.link(next);
            }
            match prev.next_mut() {
                None => break,
                Some(next) => prev = next,
            }
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
            for (i, v) in list.cursor_mut().enumerate() {
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
                let mut cursor = list.cursor_mut();
                for v in data {
                    if v != target {
                        assert_eq!(Some(v), cursor.next().copied())
                    }
                }
            } else {
                list.delete_all(&1);
            }
        }
    }
}
