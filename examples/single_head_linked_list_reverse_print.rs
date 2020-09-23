use my_algo::linked_list::{shll::LinkedList, LinearCursor, LinearCursorMut, SinglyLinkedList};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    input: Vec<u64>,
}

// 习题 2.3.3
fn main() {
    let opt = Opt::from_args();
    println!("inputs: {:?}", opt.input);

    let mut list = LinkedList::default();
    let mut cursor = list.cursor_front_mut();
    for v in opt.input.iter() {
        cursor.insert_after(*v);
        cursor.move_next();
    }

    let mut stack = Vec::new();
    for v in list.iter_mut() {
        stack.push(*v);
    }
    print!("outputs: ");
    while !stack.is_empty() {
        print!("{} ", stack.pop().unwrap())
    }
    println!()
}
