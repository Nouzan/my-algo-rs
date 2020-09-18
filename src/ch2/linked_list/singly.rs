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
    /// use my_algo::ch2::linked_list::{cdll::LinkedList, SinglyLinkedListExt};
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

    /// 删除第一次出现的最小值结点.
    /// 若表空, 则返回`None`.
    // 习题 2.3.4
    fn pop_min(&mut self) -> Option<T>
    where
        T: PartialOrd,
    {
        if !self.is_empty() {
            let mut cursor = self.cursor_front_mut(); // 指向已知最小值的游标, 由于表非空, 开始时指向首结点.
            while let Some(next) = {
                let mut pionner = cursor.as_cursor_forward(1);
                while let Some(elem) = pionner.peek() {
                    // 既然先锋作为后继不为空, `cursor`也必然不为空.
                    if *elem < *cursor.peek().unwrap() {
                        // 找到了下一个最小值
                        break;
                    }
                    pionner.move_next();
                }
                pionner.index()
            } {
                // 追上先锋.
                while cursor.index().unwrap() != next {
                    cursor.move_next();
                }
            }
            cursor.remove_current()
        } else {
            None
        }
    }

    /// 快速排序中的helper.
    /// # Panics
    /// 如果表为空则报错.
    fn partition(&mut self) -> (T, Self)
    where
        T: PartialOrd,
    {
        let flag = self.pop_front().unwrap();
        let mut rhs = Self::default();
        let mut rhs_cursor = rhs.cursor_front_mut();
        let mut lhs_cursor = self.cursor_front_mut();
        while let Some(elem) = lhs_cursor.peek() {
            if *elem >= flag {
                // 已判空, 故可直接`unwrap`.
                rhs_cursor.insert_after_as_current(lhs_cursor.remove_current().unwrap());
            } else {
                lhs_cursor.move_next();
            }
        }
        drop(rhs_cursor); // drop earlier.
        (flag, rhs)
    }

    /// (按递增序)排序.
    // 习题 2.3.6
    fn sort(&mut self)
    where
        T: PartialOrd,
    {
        if !self.is_empty() {
            let (flag, mut rhs) = self.partition();
            self.sort();
            rhs.sort();
            rhs.push_front(flag);
            self.append(&mut rhs);
        }
    }
}
