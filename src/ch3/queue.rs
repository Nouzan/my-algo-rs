//! 队列特质与队列算法.

use crate::ch2::linked_list::{cdll, SinglyLinkedList};

/// 队列特质.
/// 具有`FIFO`性质.
pub trait Queue<T> {
    /// 入队.
    /// 若入队成功则返回`None`, 否则返回`item`.
    fn enque(&mut self, item: T) -> Option<T>;

    /// 出队.
    /// 若队空则返回`None`, 否则返回队首元素.
    fn deque(&mut self) -> Option<T>;

    /// 队是否为空.
    fn is_empty(&self) -> bool;

    /// 队是否满.
    fn is_full(&self) -> bool;
}

impl<T> Queue<T> for cdll::LinkedList<T> {
    fn is_full(&self) -> bool {
        false
    }

    fn is_empty(&self) -> bool {
        SinglyLinkedList::is_empty(self)
    }

    fn enque(&mut self, item: T) -> Option<T> {
        self.push_front(item);
        None
    }

    fn deque(&mut self) -> Option<T> {
        if SinglyLinkedList::is_empty(self) {
            None
        } else {
            self.pop_back()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_queue_basic_cdll(data: Vec<i64>) {
            let mut queue = cdll::LinkedList::default();
            for elem in data.iter() {
                queue.enque(*elem);
            }
            for elem in data.iter() {
                assert_eq!(queue.deque(), Some(*elem));
            }
            assert_eq!(queue.deque(), None);
        }
    }
}
