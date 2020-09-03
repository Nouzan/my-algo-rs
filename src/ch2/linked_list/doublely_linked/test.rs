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
