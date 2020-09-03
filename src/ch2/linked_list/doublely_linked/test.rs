use super::*;

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

    let mut cursor = list.cursor();

    for elem in data.iter() {
        assert_eq!(cursor.peek(), Some(elem));
        cursor.move_next();
    }

    assert!(cursor.is_front());

    for elem in data.iter().rev() {
        cursor.move_prev();
        assert_eq!(cursor.peek(), Some(elem));
    }
}
