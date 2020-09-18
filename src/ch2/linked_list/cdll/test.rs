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

    assert!(cursor.is_ghost());

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

#[test]
fn test_cursor_mut_insert_before_failed() {
    let data = vec![0, 0];
    let mut list = LinkedList::default();
    let mut cursor = list.cursor_front_mut();

    // 逆序插入.
    for elem in data.iter() {
        cursor.insert_before_as_current(*elem);
    }

    let mut cursor = list.cursor_front();
    for (i, elem) in data.iter().rev().enumerate() {
        assert_eq!(cursor.index(), Some(i));
        assert_eq!(cursor.peek(), Some(elem));
        cursor.move_next();
    }
}

proptest! {
    #[test]
    fn test_iter_mut(mut data: Vec<i64>) {
        let mut list = LinkedList::from(data.clone());
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
        let mut list1 = LinkedList::from(data1.clone());
        let mut list2 = LinkedList::from(data2.clone());
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
        let mut list1 = LinkedList::from(data1.clone());
        let mut list2 = LinkedList::from(data2.clone());
        list1.append(&mut list2);
        data1.append(&mut data2);
        prop_assert_eq!(data1, list1);
    }

    #[test]
    fn test_as_queue(mut data: Vec<i64>) {
        let mut list = LinkedList::default();
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
        let mut list = LinkedList::default();
        for elem in data.iter().rev() {
            list.push_back(*elem);
        }
        while let Some(elem) = list.pop_front() {
            prop_assert_eq!(elem, data.pop().unwrap())
        }
        prop_assert!(list.is_empty());
        prop_assert!(data.is_empty());
    }
}

proptest! {
    #[test]
    fn test_last(data: Vec<usize>, n: usize) {
        let list = LinkedList::from(data.clone());
        if 1 <= n && n <= data.len() {
            assert_eq!(list.last(n), Some(&data[data.len() - n]));
        } else {
            assert_eq!(list.last(n), None);
        }
    }

    #[test]
    fn test_find(data: String, pattern: String) {
        let list = LinkedList::from(Vec::from(data.clone()));
        let cursor = list.find(&LinkedList::from(Vec::from(pattern.clone())));
        let idx = match cursor.map(|cur| cur.index()) {
            Some(Some(idx)) => Some(idx),
            Some(None) => Some(0),
            _ => None,
        };
        prop_assert_eq!(idx, data.find(&pattern));
    }

    #[test]
    fn test_dedup(mut data: Vec<i64>) {
        let mut list = LinkedList::from(data.clone());
        list.dedup();
        data.dedup();
        assert_eq!(list, data);
    }

    #[test]
    fn test_delete_between(data: Vec<isize>, a: isize, b: isize) {
        let mut list = LinkedList::from(data);
        list.delete_between(&a, &b);
        for v in list.iter() {
            prop_assert!(!(a <= *v && *v < b));
        }
    }

    #[test]
    fn test_sort(mut data: Vec<isize>) {
        let mut list = LinkedList::from(data.clone());
        list.sort();
        data.sort_unstable();
        prop_assert_eq!(data, list);
    }

    #[test]
    fn test_reverse(mut data: Vec<isize>) {
        let mut list: LinkedList<_> = data.clone().into();
        list.reverse();
        data.reverse();
        prop_assert_eq!(list, data);
    }

    #[test]
    fn test_basic(data: Vec<isize>) {
        let list: LinkedList<_> = data.clone().into();
        prop_assert_eq!(data, Vec::from(list));
    }

    #[test]
    fn test_delete_all(data: Vec<isize>) {
        let mut list: LinkedList<_> = data.clone().into();
        if !data.is_empty() {
            let target = data[0];
            list.delete_all(&target);
            let mut iter = list.iter_mut();
            for v in data {
                if v != target {
                    prop_assert_eq!(Some(v), iter.next().copied())
                }
            }
        } else {
            list.delete_all(&1);
        }
    }

    #[test]
    fn test_cursor_mut_insert_before(data: Vec<isize>) {
        // 逆序插入
        let mut list = LinkedList::default();
        let mut cursor = list.cursor_front_mut();

        for v in data.iter().rev() {
            prop_assert_eq!(cursor.insert_before_as_current(*v), None);
        }

        prop_assert_eq!(&list, &data);

        // 通过`insert_before`插入.
        let mut list = LinkedList::default();
        let mut cursor = list.cursor_front_mut();

        if !data.is_empty() {
            prop_assert_eq!(cursor.insert_before(data[data.len() - 1]), None);
        }

        if data.len() > 1 {
            for v in data[0..(data.len() - 1)].iter() {
                prop_assert_eq!(cursor.insert_before(*v), None);
            }
        }

        prop_assert_eq!(&list, &data);
    }

    #[test]
    fn test_cursor_mut_insert_after(data: Vec<isize>) {
        // 顺序插入(通过`insert_after_as_current`)
        let mut list = LinkedList::default();
        let mut cursor = list.cursor_front_mut();

        // 不变式: 游标始终指向尾结点的后继. 因此调用`insert_after`将会始终在末尾插入新元素.
        // 开始时表为空, `cursor.prev`为头结点(可看作尾结点), 因此游标指向尾结点的后继`None`.
        for v in data.iter() {
            // 在尾结点的后继之后插入新的元素(将会直接插入作为尾结点的后继), 插入后游标指向新插入的结点(即新的尾结点).
            prop_assert_eq!(cursor.insert_after_as_current(*v), None);
        }

        // 通过`insert_after`插入.
        let mut list = LinkedList::default();
        let mut cursor = list.cursor_front_mut();

        if !data.is_empty() {
            prop_assert_eq!(cursor.insert_after(data[0]), None);
        }

        if data.len() > 1 {
            for v in data[1..].iter().rev() {
                prop_assert_eq!(cursor.insert_after(*v), None);
            }
        }
        prop_assert_eq!(list, data);
    }

    #[test]
    fn test_pop_min(mut data: Vec<isize>) {
        let mut list: LinkedList<_> = data.clone().into();
        let min = list.pop_min();
        let mut min_idx = None;
        if let Some(min) = min {
            for (idx, v) in data.iter().enumerate() {
                prop_assert!(min <= *v);
                if min_idx.is_none() && min == *v {
                    min_idx = Some(idx);
                }
            };
        } else {
            prop_assert!(data.is_empty());
        }
        if let Some(idx) = min_idx {
            data.remove(idx);
        }

        prop_assert_eq!(list, data);
    }
}
