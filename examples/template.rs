use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    input: u64,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
