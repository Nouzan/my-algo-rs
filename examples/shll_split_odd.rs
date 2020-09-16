use my_algo::ch2::linked_list::shll::{utils::split_odd, LinkedList};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    input: Vec<u64>,
}

fn main() {
    let opt = Opt::from_args();
    println!("inputs: {:?}", opt.input);

    let mut list = LinkedList::from(opt.input);
    let odd = split_odd(&mut list);

    println!("{:?}", odd);
    println!("{:?}", list);
}
