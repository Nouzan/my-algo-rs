use my_algo::fib_recurrence;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    input: usize,
}

fn main() {
    let opt = Opt::from_args();
    let result = fib_recurrence(opt.input);
    println!("{}", result);
}
