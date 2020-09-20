//! 栈特质与栈算法.

use super::Queue;
use crate::ch2::{
    linked_list::{cdll, shll, SinglyLinkedList},
    List,
};
use crate::vec::MyVec;

/// 栈特质.
/// 具有`FILO`性质.
pub trait Stack {
    /// 栈元素.
    type Elem;

    /// 入栈.
    /// 若入栈成功则返回`None`, 否则返回`item`.
    fn push(&mut self, elem: Self::Elem) -> Option<Self::Elem>;

    /// 出栈.
    /// 若栈空则返回`None`, 否则返回栈顶元素.
    fn pop(&mut self) -> Option<Self::Elem>;

    /// 栈是否为空.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 栈是否满.
    fn is_full(&self) -> bool {
        self.cap().map_or(false, |cap| self.len() == cap)
    }

    /// 栈容量.
    fn cap(&self) -> Option<usize>;

    /// 栈大小.
    fn len(&self) -> usize;
}

impl<T> Stack for Vec<T> {
    type Elem = T;

    fn is_full(&self) -> bool {
        false
    }

    fn cap(&self) -> Option<usize> {
        None
    }

    fn len(&self) -> usize {
        List::len(self)
    }

    fn is_empty(&self) -> bool {
        List::is_empty(self)
    }

    fn push(&mut self, item: T) -> Option<T> {
        self.push(item);
        None
    }

    fn pop(&mut self) -> Option<T> {
        self.pop()
    }
}

impl<T> Stack for MyVec<T> {
    type Elem = T;

    fn is_full(&self) -> bool {
        false
    }

    fn cap(&self) -> Option<usize> {
        None
    }

    fn len(&self) -> usize {
        List::len(self)
    }

    fn is_empty(&self) -> bool {
        List::is_empty(self)
    }

    fn push(&mut self, item: T) -> Option<T> {
        self.insert(List::len(self), item).unwrap();
        None
    }

    fn pop(&mut self) -> Option<T> {
        if List::is_empty(self) {
            None
        } else {
            self.delete(List::len(self) - 1).ok()
        }
    }
}

impl<T> Stack for cdll::LinkedList<T> {
    type Elem = T;

    fn is_full(&self) -> bool {
        false
    }

    fn cap(&self) -> Option<usize> {
        None
    }

    fn len(&self) -> usize {
        SinglyLinkedList::len(self)
    }

    fn is_empty(&self) -> bool {
        SinglyLinkedList::is_empty(self)
    }

    fn push(&mut self, item: T) -> Option<T> {
        self.push_front(item);
        None
    }

    fn pop(&mut self) -> Option<T> {
        if SinglyLinkedList::is_empty(self) {
            None
        } else {
            self.pop_front()
        }
    }
}

impl<T> Stack for shll::LinkedList<T> {
    type Elem = T;

    fn is_full(&self) -> bool {
        false
    }

    fn cap(&self) -> Option<usize> {
        None
    }

    fn len(&self) -> usize {
        SinglyLinkedList::len(self)
    }

    fn is_empty(&self) -> bool {
        SinglyLinkedList::is_empty(self)
    }

    fn push(&mut self, item: T) -> Option<T> {
        self.push_front(item);
        None
    }

    fn pop(&mut self) -> Option<T> {
        if SinglyLinkedList::is_empty(self) {
            None
        } else {
            self.pop_front()
        }
    }
}

/// 切片栈.
pub struct SliceStack<'a, T> {
    top: usize,
    slice: &'a mut [T],
}

impl<'a, T> From<&'a mut [T]> for SliceStack<'a, T> {
    fn from(slice: &'a mut [T]) -> Self {
        Self { top: 0, slice }
    }
}

impl<'a, T: Clone> Stack for SliceStack<'a, T> {
    type Elem = T;

    fn cap(&self) -> Option<usize> {
        Some(self.slice.len())
    }

    fn len(&self) -> usize {
        self.top
    }

    fn push(&mut self, elem: Self::Elem) -> Option<Self::Elem> {
        if self.is_full() {
            Some(elem)
        } else {
            self.slice[self.top] = elem;
            self.top += 1;
            None
        }
    }

    fn pop(&mut self) -> Option<Self::Elem> {
        if self.is_empty() {
            None
        } else {
            self.top -= 1;
            Some(self.slice[self.top].clone())
        }
    }
}

/// 双栈.
pub struct DoubleStack<S>(S, S);

impl<S: Stack + Default> Default for DoubleStack<S> {
    fn default() -> Self {
        Self(S::default(), S::default())
    }
}

fn full_or_cap<S: Stack>(s: &S, maybe_cap: Option<usize>) -> bool {
    let cap = match (s.cap(), maybe_cap) {
        (Some(cap0), Some(cap1)) => cap0.min(cap1),
        (Some(cap0), None) => cap0,
        (None, Some(cap1)) => cap1,
        _ => return false,
    };
    s.len() == cap
}

/// 使用双栈结构实现队列.
// 习题 3.2.3
impl<S: Stack> Queue for DoubleStack<S> {
    type Elem = S::Elem;

    fn is_empty(&self) -> bool {
        self.0.is_empty() && self.1.is_empty()
    }

    fn is_full(&self) -> bool {
        if self.1.is_empty() {
            self.1.is_full() || self.0.cap().map_or(false, |cap| cap == 0)
        } else {
            full_or_cap(&self.0, self.1.cap())
        }
    }

    fn enque(&mut self, elem: Self::Elem) -> Option<Self::Elem> {
        if self.is_full() {
            Some(elem)
        } else {
            if full_or_cap(&self.0, self.1.cap()) && self.1.is_empty() {
                while let Some(elem) = self.0.pop() {
                    self.1.push(elem);
                }
            }
            self.0.push(elem)
        }
    }

    fn deque(&mut self) -> Option<Self::Elem> {
        if self.1.is_empty() {
            while let Some(elem) = self.0.pop() {
                self.1.push(elem);
            }
        }
        if self.1.is_empty() {
            None
        } else {
            self.1.pop()
        }
    }
}

impl<S: Stack> StackExt for S {}

/// 栈扩展特质.
/// 实现了一些栈算法.
pub trait StackExt: Stack {}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_queue_basic_double_stack() {
        let data = vec![1, 2, 3, 4];
        let mut v1 = vec![0; 2];
        let mut v2 = vec![0; 2];
        let s1 = SliceStack::from(v1.as_mut_slice());
        let s2 = SliceStack::from(v2.as_mut_slice());
        let mut queue = DoubleStack(s1, s2);
        for elem in data.iter() {
            assert_eq!(queue.enque(*elem), None);
        }
        for elem in data.iter() {
            assert_eq!(queue.deque(), Some(*elem));
        }
        assert_eq!(queue.deque(), None);
    }

    proptest! {
        #[test]
        fn test_stack_basic_vec(data: Vec<i64>) {
            let mut stack = Vec::new();
            for elem in data.iter() {
                stack.push(*elem);
            }
            for elem in data.iter().rev() {
                assert_eq!(stack.pop(), Some(*elem));
            }
            assert_eq!(stack.pop(), None);
        }

        #[test]
        fn test_stack_basic_myvec(data: Vec<i64>) {
            let mut stack = MyVec::new();
            for elem in data.iter() {
                stack.push(*elem);
            }
            for elem in data.iter().rev() {
                assert_eq!(stack.pop(), Some(*elem));
            }
            assert_eq!(stack.pop(), None);
        }

        #[test]
        fn test_stack_basic_shll(data: Vec<i64>) {
            let mut stack = shll::LinkedList::default();
            for elem in data.iter() {
                stack.push(*elem);
            }
            for elem in data.iter().rev() {
                assert_eq!(stack.pop(), Some(*elem));
            }
            assert_eq!(stack.pop(), None);
        }

        #[test]
        fn test_stack_basic_cdll(data: Vec<i64>) {
            let mut stack = cdll::LinkedList::default();
            for elem in data.iter() {
                stack.push(*elem);
            }
            for elem in data.iter().rev() {
                assert_eq!(stack.pop(), Some(*elem));
            }
            assert_eq!(stack.pop(), None);
        }

        #[test]
        fn test_stack_basic_slice_stack(data: Vec<i64>) {
            let mut v = vec![0;data.len()];
            let mut stack = SliceStack::from(v.as_mut_slice());
            for elem in data.iter() {
                stack.push(*elem);
            }
            for elem in data.iter().rev() {
                assert_eq!(stack.pop(), Some(*elem));
            }
            assert_eq!(stack.pop(), None);
        }
    }
}
