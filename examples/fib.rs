use my_algo::ch1::fib;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    input: u64,
}

fn main() {
    let opt = Opt::from_args();
    let result = fib(opt.input);
    println!("{}", result);
}
