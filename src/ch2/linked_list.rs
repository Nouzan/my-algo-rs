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
    /// 使用`cursor`遍历链表, 递归地删除等于`x`的元素.
    /// # Correctness
    /// 要求`cursor`当前所指结点为`None`或其值不为`x`.
    fn delete_all_inner(mut cursor: CursorMut<T>, x: &T) {
        if cursor.peek().is_some() {
            while let Some(elem) = cursor.peek_next() {
                if *elem == *x {
                    let node = cursor.as_mut().unwrap(); // 这里必然为`Some`, 否则`peek_next`将不会返回`Some`.
                    let mut next = *node.next.take().unwrap(); // 同上.
                    node.next = next.next.take();
                } else {
                    break;
                }
            }
            cursor.next();
            Self::delete_all_inner(cursor, x)
        }
    }

    /// 删除所有值等于`x`的元素.
    pub fn delete_all(&mut self, x: &T) {
        let mut cursor = self.cursor_mut();
        while let Some(elem) = cursor.peek() {
            if *elem == *x {
                let mut head = self.head.take().unwrap(); // 这里必然为`Some`, 否则`peek`将不会返回`Some`.
                self.head = head.next.take();
                cursor = self.cursor_mut();
            } else {
                break;
            }
        }
        Self::delete_all_inner(cursor, x);
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
