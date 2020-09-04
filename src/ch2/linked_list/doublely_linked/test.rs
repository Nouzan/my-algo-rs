use super::*;
use proptest::prelude::*;

#[test]
fn test_drop() {
    struct Foo;

    impl Drop for Foo {
        fn drop(&mut self) {
            println!("drop!");
        }
    }

    let mut list = LinkedList::default();

    for _ in 0..10 {
        list.push_front(Foo);
    }

    assert!(list.pop_front().is_some());
}

#[test]
fn test_cursor() {
    let data = vec![1, 2, 3, 4, 5];
    let mut list = LinkedList::default();

    for elem in data.iter().rev() {
        list.push_front(*elem);
    }

    let mut cursor = list.cursor_front();

    for elem in data.iter() {
        assert_eq!(cursor.peek(), Some(elem));
        cursor.move_next();
    }

    assert!(cursor.is_front_or_empty());

    for elem in data.iter().rev() {
        cursor.move_prev();
        assert_eq!(cursor.peek(), Some(elem));
    }
}

#[test]
fn test_cursor_mut_remove() {
    let data = vec![1, 2, 3, 4, 5];

    let mut list = LinkedList::default();

    for elem in data.iter().rev() {
        list.push_front(*elem);
    }

    let mut cursor = list.cursor_front_mut();
    cursor.move_next();
    cursor.move_next();
    cursor.move_next();

    let mut idx = 0;
    while cursor.peek().is_some() {
        assert_eq!(cursor.remove_current(), Some(data[(idx + 3) % 5]));
        idx += 1;
    }
}

proptest! {
    #[test]
    fn test_as_queue(mut data: Vec<i64>) {
        let mut list = LinkedList::default();
        for elem in data.iter().rev() {
            list.push_front(*elem);
        }
        while let Some(elem) = list.pop_back() {
            assert_eq!(elem, data.pop().unwrap())
        }
        assert!(list.is_empty());
        assert!(data.is_empty());
    }

    #[test]
    fn test_as_queue_opposite(mut data: Vec<i64>) {
        let mut list = LinkedList::default();
        for elem in data.iter().rev() {
            list.push_back(*elem);
        }
        while let Some(elem) = list.pop_front() {
            assert_eq!(elem, data.pop().unwrap())
        }
        assert!(list.is_empty());
        assert!(data.is_empty());
    }

    #[test]
    fn test_cursor_mut_insert_before(data: Vec<i64>) {
        let mut list = LinkedList::default();
        let mut cursor = list.cursor_front_mut();

        // 逆序插入.
        for elem in data.iter() {
            cursor.insert_before(*elem);
            cursor.move_prev();
        }

        let mut cursor = list.cursor_front();
        for (i, elem) in data.iter().rev().enumerate() {
            prop_assert_eq!(cursor.index(), Some(i));
            prop_assert_eq!(cursor.peek(), Some(elem));
            cursor.move_next();
        }
    }

    #[test]
    fn test_cursor_mut_insert_after(data: Vec<i64>) {
        let mut list = LinkedList::default();
        let mut cursor = list.cursor_front_mut();

        // 顺序插入.
        for elem in data.iter() {
            cursor.insert_after(*elem);
            cursor.move_next();
        }

        let mut cursor = list.cursor_front();
        for (i, elem) in data.iter().enumerate() {
            prop_assert_eq!(cursor.index(), Some(i));
            prop_assert_eq!(cursor.peek(), Some(elem));
            cursor.move_next();
        }
    }
}
