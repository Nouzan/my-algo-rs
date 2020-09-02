use my_algo::ch2::linked_list::single_head::LinkedList;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    input: Vec<u64>,
}

// 把链表分解为奇链和偶链.(分别包含原来链表中的奇数位置结点和偶数位置结点)
// 习题 2.3.10
fn split_odd<T>(list: &mut LinkedList<T>) -> LinkedList<T> {
    let mut cursor = list.cursor_mut();
    let mut odd = LinkedList::default();
    let mut odd_cursor = odd.cursor_mut();
    let mut idx = 0;
    while cursor.peek().is_some() {
        idx += 1;
        if idx % 2 == 1 {
            odd_cursor.insert_after(cursor.remove_current().unwrap());
            odd_cursor.move_next();
        } else {
            cursor.move_next();
        }
    }
    odd
}

fn main() {
    let opt = Opt::from_args();
    println!("inputs: {:?}", opt.input);

    let mut list = LinkedList::from(opt.input);
    let odd = split_odd(&mut list);

    println!("{:?}", odd);
    println!("{:?}", list);
}
