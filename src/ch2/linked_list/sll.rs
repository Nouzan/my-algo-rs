type Link<T> = Option<Box<Node<T>>>;

/// 单链表结点.
pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Box<Self> {
        Box::new(Self { elem, next: None })
    }
}

/// 单链表.
pub struct LinkedList<T> {
    head: Link<T>,
}

/// 单链表游标.
pub struct CursorMut<'a, T: 'a>(Option<&'a mut Node<T>>);

impl<T> LinkedList<T> {
    pub fn node_mut(link: &mut Link<T>) -> Option<&mut Node<T>> {
        link.as_mut().map(|node| &mut **node)
    }

    pub fn cursor_mut(&mut self) -> CursorMut<T> {
        CursorMut(Self::node_mut(&mut self.head))
    }

    pub fn push_front(&mut self, elem: T) {
        let mut node = Node::new(elem);
        if let Some(head) = self.head.take() {
            node.next = Some(head);
        }
        self.head = Some(node);
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self { head: None }
    }
}

impl<'a, T> CursorMut<'a, T> {
    pub fn peek(&self) -> Option<&T> {
        if let Some(node) = &self.0 {
            Some(&node.elem)
        } else {
            None
        }
    }

    pub fn peek_next(&self) -> Option<&T> {
        if let Some(node) = &self.0 {
            node.next.as_ref().map(|node| &node.elem)
        } else {
            None
        }
    }

    pub fn as_mut(&mut self) -> Option<&mut &'a mut Node<T>> {
        self.0.as_mut()
    }

    pub fn into_inner(self) -> Option<&'a mut Node<T>> {
        self.0
    }
}

impl<'a, T> Iterator for CursorMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|node| {
            self.0 = LinkedList::node_mut(&mut node.next);
            &mut node.elem
        })
    }
}

use std::cmp::PartialEq;

impl<T: PartialEq> LinkedList<T> {
    /// 使用`Link`遍历链表, 若当前结点值等于`x`则将`Link`改为指向它的后继.
    fn delete_all_inner(mut link: &mut Link<T>, x: &T) {
        if let Some(node) = Self::node_mut(link) {
            if node.elem == *x {
                let next = link.take().unwrap().next;
                *link = next;
            } else {
                link = &mut Self::node_mut(link).unwrap().next;
            }
            Self::delete_all_inner(link, x)
        }
    }

    /// 删除所有值等于`x`的元素(使用`Link`).
    pub fn delete_all(&mut self, x: &T) {
        Self::delete_all_inner(&mut self.head, x)
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
