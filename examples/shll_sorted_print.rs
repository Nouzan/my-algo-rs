use my_algo::ch2::linked_list::single_head::LinkedList;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    input: Vec<i64>,
}

// 带头结点的单链表
// 习题 2.3.9
fn main() {
    let opt = Opt::from_args();
    let mut list = LinkedList::from(opt.input);
    list.sort();
    for elem in list.iter() {
        print!("{} ", *elem)
    }
    println!();
}
