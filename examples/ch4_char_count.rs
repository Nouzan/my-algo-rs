use my_algo::ch4::coding_tree::char_count;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let path = opt.input;
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    for (c, count) in char_count(&contents) {
        println!("{:?}: {}", c, count);
    }

    Ok(())
}
