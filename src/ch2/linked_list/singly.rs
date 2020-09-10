use super::*;

impl<T, L> SinglyLinkedListExt<T> for L where L: SinglyLinkedList<T> {}

pub trait SinglyLinkedListExt<T>: SinglyLinkedList<T> {
    /// 就地逆置.
    // 习题 2.3.5
    fn reverse(&mut self) {
        // a -> b -> c
        // a, b -> c
        // b -> a, c
        // c -> b -> a
        if !self.is_empty() {
            let mut left = Self::default();
            while let Some(elem) = self.pop_front() {
                left.push_front(elem);
            }
            *self = left;
        }
    }

    /// 获取倒数第n个元素.
    /// 若表不够长, 则返回`None`.
    /// # Examples
    /// ```
    /// use my_algo::ch2::linked_list::{SinglyLinkedListExt, shll::LinkedList};
    ///
    /// let list = LinkedList::from(vec![5, 4, 3, 2, 1]);
    /// assert_eq!(list.last(1), Some(&1));
    /// assert_eq!(list.last(0), None);
    /// assert_eq!(list.last(6), None);
    /// assert_eq!(list.last(5), Some(&5));
    /// ```
    // 习题 2.3.21
    fn last(&self, n: usize) -> Option<&T> {
        if n == 0 {
            return None;
        }
        let mut lcur = self.cursor_front();
        let mut rcur = self.cursor_front();
        let mut flag = false;
        while rcur.peek().is_some() {
            if flag {
                lcur.move_next();
            }
            if rcur.index().unwrap() - lcur.index().unwrap() >= n - 1 {
                flag = true;
            }
            rcur.move_next();
        }
        if flag {
            lcur.into_ref()
        } else {
            None
        }
    }

    /// 删除所有值等于`x`的元素.
    fn delete_all(&mut self, x: &T)
    where
        T: PartialEq,
    {
        let mut cursor = self.cursor_front_mut();
        while let Some(current) = cursor.peek() {
            if *current == *x {
                cursor.remove_current();
            }
            cursor.move_next();
        }
    }

    /// 去除连续重复的元素.
    ///
    /// 若单链表是有序的, 这将去除所有重复元素.
    /// # Examples
    /// ```
    /// use my_algo::ch2::linked_list::{shll::LinkedList, SinglyLinkedListExt};
    ///
    /// let mut list = LinkedList::from(vec![1, 2, 2, 3, 2]);
    /// list.dedup();
    /// assert_eq!(list, vec![1, 2, 3, 2]);
    /// ```
    // 习题 2.3.12
    fn dedup(&mut self)
    where
        T: PartialEq,
    {
        let mut cursor = self.cursor_front_mut();
        while let Some(is_dedup) = {
            let flag = cursor
                .as_cursor_forward(1)
                .peek()
                .map(|elem| *elem == *cursor.as_cursor().peek().unwrap());
            flag
        } {
            if is_dedup {
                cursor.remove_current();
            } else {
                cursor.move_next();
            }
        }
    }
}
