use my_algo::ch2::linked_list::shll::{utils::dedup_by_abs, LinkedList};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    input: Vec<isize>,
}

fn main() {
    let opt = Opt::from_args();
    println!("inputs: {:?}", opt.input);
    let n = opt
        .input
        .iter()
        .max_by(|x, y| x.abs().cmp(&y.abs()))
        .copied();
    let mut list = LinkedList::from(opt.input);
    if let Some(n) = n {
        dedup_by_abs(&mut list, n.abs() as usize);
    }

    println!("{:?}", list);
}
