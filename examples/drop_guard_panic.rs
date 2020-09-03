use std::collections::LinkedList;

struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("drop!");
        panic!("test drop guard");
    }
}

// Drop Guard只能保证在一次panic后继续执行, 不能保证两次或更多panic后仍能继续执行.
fn main() {
    let mut list = LinkedList::default();
    list.push_front(Foo);
    list.push_front(Foo);
    list.push_front(Foo);
}
