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

    let mut list = CircularDoublyLinkedList::default();

    for _ in 0..10 {
        list.push_front(Foo);
    }

    assert!(list.pop_front().is_some());
}

#[test]
fn test_cursor() {
    let data = vec![1, 2, 3, 4, 5];
    let mut list = CircularDoublyLinkedList::default();

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

    let mut list = CircularDoublyLinkedList::default();

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
    fn test_iter_mut(mut data: Vec<i64>) {
        let mut list = CircularDoublyLinkedList::from(data.clone());
        for elem in list.iter_mut() {
            if *elem < i64::MAX {
                *elem += 1;
            }
        }
        for elem in data.iter_mut() {
            if *elem < i64::MAX {
                *elem += 1;
            }
        }

        prop_assert_eq!(data, list);
    }

    #[test]
    fn test_eq(mut data1: Vec<i64>, mut data2: Vec<i64>) {
        let mut list1 = CircularDoublyLinkedList::from(data1.clone());
        let mut list2 = CircularDoublyLinkedList::from(data2.clone());
        prop_assert_eq!(&list1, &data1);
        list1.append(&mut list2);
        if data2.is_empty() {
            prop_assert_eq!(&list1, &data1);
        } else {
            prop_assert_ne!(list1, data1);
        }
    }

    #[test]
    fn test_append(mut data1: Vec<i64>, mut data2: Vec<i64>) {
        let mut list1 = CircularDoublyLinkedList::from(data1.clone());
        let mut list2 = CircularDoublyLinkedList::from(data2.clone());
        list1.append(&mut list2);
        data1.append(&mut data2);
        prop_assert_eq!(data1, list1);
    }

    #[test]
    fn test_as_queue(mut data: Vec<i64>) {
        let mut list = CircularDoublyLinkedList::default();
        for elem in data.iter().rev() {
            list.push_front(*elem);
        }
        while let Some(elem) = list.pop_back() {
            prop_assert_eq!(elem, data.pop().unwrap())
        }
        prop_assert!(list.is_empty());
        prop_assert!(data.is_empty());
    }

    #[test]
    fn test_as_queue_opposite(mut data: Vec<i64>) {
        let mut list = CircularDoublyLinkedList::default();
        for elem in data.iter().rev() {
            list.push_back(*elem);
        }
        while let Some(elem) = list.pop_front() {
            prop_assert_eq!(elem, data.pop().unwrap())
        }
        prop_assert!(list.is_empty());
        prop_assert!(data.is_empty());
    }

    #[test]
    fn test_cursor_mut_insert_before(data: Vec<i64>) {
        let mut list = CircularDoublyLinkedList::default();
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
        let mut list = CircularDoublyLinkedList::default();
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
