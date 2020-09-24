//! 队列特质与队列算法.

use crate::ch2::linked_list::{cdll, SinglyLinkedList};

/// 队列特质.
/// 具有`FIFO`性质.
pub trait Queue {
    type Elem;

    /// 入队.
    /// 若入队成功则返回`None`, 否则返回`elem`.
    fn enqueue(&mut self, elem: Self::Elem) -> Option<Self::Elem>;

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

/// (有限容量的)循环队列.
pub struct CircularQueue<T> {
    /// 指向队首.
    /// 若队尾的下一个位置与队首重叠, 则需要根据`full`来判断是否指向队首.
    front: usize,

    /// 指向队尾的下一个位置.
    back: usize,
    full: bool,
    list: Vec<Option<T>>,
}

impl<T> CircularQueue<T> {
    pub fn new(size: usize) -> Self {
        let mut list = Vec::with_capacity(size);
        for _ in 0..size {
            list.push(None);
        }
        Self {
            front: 0,
            back: 0,
            full: false,
            list,
        }
    }
}

impl<T> Queue for CircularQueue<T> {
    type Elem = T;

    fn is_empty(&self) -> bool {
        !self.full && self.front == self.back
    }

    fn is_full(&self) -> bool {
        self.full
    }

    fn enqueue(&mut self, elem: Self::Elem) -> Option<Self::Elem> {
        if self.is_full() {
            Some(elem)
        } else {
            self.list[self.back] = Some(elem);
            self.back = (self.back + 1) % self.list.len();
            if self.front == self.back {
                self.full = true;
            }
            None
        }
    }

    fn dequeue(&mut self) -> Option<Self::Elem> {
        if self.is_empty() {
            None
        } else {
            self.full = false;
            let elem = self.list[self.front].take();
            self.front = (self.front + 1) % self.list.len();
            elem
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

        #[test]
        fn test_queue_basic_circular_queue(data: Vec<i64>) {
            let mut queue = CircularQueue::new(data.len());
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
