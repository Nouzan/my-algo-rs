//! 栈特质与栈算法.

use crate::ch2::{
    linked_list::{cdll, shll, SinglyLinkedList},
    List,
};
use crate::vec::MyVec;

/// 栈特质.
/// 具有`FILO`性质.
pub trait Stack<T> {
    /// 入栈.
    /// 若入栈成功则返回`None`, 否则返回`item`.
    fn push(&mut self, item: T) -> Option<T>;

    /// 出栈.
    /// 若栈空则返回`None`, 否则返回栈顶元素.
    fn pop(&mut self) -> Option<T>;

    /// 栈是否为空.
    fn is_empty(&self) -> bool;

    /// 栈是否满.
    fn is_full(&self) -> bool;
}

impl<T> Stack<T> for Vec<T> {
    fn is_full(&self) -> bool {
        false
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

impl<T> Stack<T> for MyVec<T> {
    fn is_full(&self) -> bool {
        false
    }

    fn is_empty(&self) -> bool {
        List::is_empty(self)
    }

    fn push(&mut self, item: T) -> Option<T> {
        self.insert(self.len(), item).unwrap();
        None
    }

    fn pop(&mut self) -> Option<T> {
        if List::is_empty(self) {
            None
        } else {
            self.delete(self.len() - 1).ok()
        }
    }
}

impl<T> Stack<T> for cdll::LinkedList<T> {
    fn is_full(&self) -> bool {
        false
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

impl<T> Stack<T> for shll::LinkedList<T> {
    fn is_full(&self) -> bool {
        false
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

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

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
    }
}
