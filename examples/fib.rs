use my_algo::fib;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    input: usize,
}

fn main() {
    let opt = Opt::from_args();
    let result = fib(opt.input);
    println!("{}", result);
}
