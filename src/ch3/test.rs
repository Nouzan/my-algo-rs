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
