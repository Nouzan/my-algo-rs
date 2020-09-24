//! 队列特质与队列算法.

use crate::ch2::linked_list::{cdll, SinglyLinkedList};

/// 队列特质.
/// 具有`FIFO`性质.
pub trait Queue {
    type Elem;

    /// 入队.
    /// 若入队成功则返回`None`, 否则返回`item`.
    fn enqueue(&mut self, item: Self::Elem) -> Option<Self::Elem>;

    /// 出队.
    /// 若队空则返回`None`, 否则返回队首元素.
    fn dequeue(&mut self) -> Option<Self::Elem>;

    /// 队是否为空.
    fn is_empty(&self) -> bool;

    /// 队是否满.
    fn is_full(&self) -> bool;
}

impl<T> Queue for cdll::LinkedList<T> {
    type Elem = T;

    fn is_full(&self) -> bool {
        false
    }

    fn is_empty(&self) -> bool {
        SinglyLinkedList::is_empty(self)
    }

    fn enqueue(&mut self, item: T) -> Option<T> {
        self.push_front(item);
        None
    }

    fn dequeue(&mut self) -> Option<T> {
        if SinglyLinkedList::is_empty(self) {
            None
        } else {
            self.pop_back()
        }
    }
}

impl<S: Queue> QueueExt for S {}

/// 队列扩展特质.
/// 实现了一些队列算法.
pub trait QueueExt: Queue {}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_queue_basic_cdll(data: Vec<i64>) {
            let mut queue = cdll::LinkedList::default();
            for elem in data.iter() {
                queue.enqueue(*elem);
            }
            for elem in data.iter() {
                assert_eq!(queue.dequeue(), Some(*elem));
            }
            assert_eq!(queue.dequeue(), None);
        }
    }
}
