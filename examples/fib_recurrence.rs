use my_algo::ch1::fib_recurrence;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    input: u64,
}

fn main() {
    let opt = Opt::from_args();
    let result = fib_recurrence(opt.input);
    println!("{}", result);
}
