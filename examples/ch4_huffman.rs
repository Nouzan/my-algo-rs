use my_algo::ch4::coding_tree::{HuffmanChar, HuffmanCodingTree};
use my_algo::ch4::linked_binary_tree::LinkedBinaryTree;
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
    let tree = HuffmanCodingTree::<LinkedBinaryTree<HuffmanChar>>::new(&contents).unwrap();
    let (encoded, len) = tree.encoded();
    println!("encoded = {:?}, len = {}B", encoded, len / 8);
    assert_eq!(tree.decode(), contents);
    Ok(())
}
