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
}
