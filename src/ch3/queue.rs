//! 队列特质与队列算法.

use super::Stack;
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
pub trait QueueExt: Queue {
    /// 使用指定栈翻转队列元素.
    /// # Panics
    /// 算法假设栈不会发生上溢, 否则报错, 且队列元素将有可能丢失.
    fn reverse_by<S: Stack<Elem = Self::Elem> + Default>(&mut self) {
        let mut stack = S::default();
        while let Some(elem) = self.dequeue() {
            if stack.push(elem).is_some() {
                panic!("栈上溢.");
            }
        }

        while let Some(elem) = stack.pop() {
            self.enqueue(elem);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_circular_queue() {
        let mut queue = CircularQueue::new(5);
        assert_eq!(queue.enqueue(1), None);
        assert_eq!(queue.enqueue(2), None);
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.enqueue(3), None);
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.enqueue(4), None);
        assert_eq!(queue.enqueue(5), None);
        assert_eq!(queue.enqueue(6), None);
        assert_eq!(queue.enqueue(7), None);
        assert_eq!(queue.enqueue(8), Some(8));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.enqueue(8), None);
        assert_eq!(queue.dequeue(), Some(4));
        assert_eq!(queue.dequeue(), Some(5));
        assert_eq!(queue.dequeue(), Some(6));
        assert_eq!(queue.dequeue(), Some(7));
        assert_eq!(queue.dequeue(), Some(8));
        assert_eq!(queue.dequeue(), None);
    }

    proptest! {
        #[test]
        fn test_reverse_by(mut data: Vec<usize>) {
            let mut queue = cdll::LinkedList::default();
            for elem in data.iter() {
                queue.enqueue(*elem);
            }
            queue.reverse_by::<Vec<_>>();
            data.reverse();
            for elem in data.iter() {
                assert_eq!(Some(*elem), queue.dequeue())
            }
            assert!(Queue::is_empty(&queue));
        }

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
