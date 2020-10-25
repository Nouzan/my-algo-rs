use my_algo::ch4::avlt::AVLTreeMap;
use my_algo::ch4::bst::TreeMap;
use my_algo::ch4::bt::BTreeMap as MyBTreeMap;
use my_algo::ch4::coding_tree::HuffmanCodingTree;
use my_algo::ch4::complete_heap::CompleteMaxHeap;
use my_algo::ch4::doubly_linked_binary_tree::DoublyLinkedBinaryTree;
use my_algo::ch4::left_heap::LeftHeap;
use my_algo::ch4::linked_binary_tree::LinkedBinaryTree;
use my_algo::ch4::rbt::RBTreeMap;
use my_algo::ch4::st::SplayTreeMap;
use my_algo::ch4::vec_binary_tree::VecBinaryTree;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::clap::arg_enum;
use structopt::StructOpt;

const MBTM_ARG: usize = 12;

arg_enum! {
    #[derive(Debug)]
    enum Tree {
        Lbt,
        Dlbt,
        Vbt,
    }
}

arg_enum! {
    #[derive(Debug)]
    enum Heap {
        Ch,
        Lh,
    }
}

arg_enum! {
    #[derive(Debug)]
    enum Map {
        Mbtm,
        Btm,
        Hm,
        Tm,
        Avl,
        Stm,
        Rbtm,
    }
}

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(long, possible_values = &Tree::variants(), case_insensitive = true, default_value = "lbt")]
    tree: Tree,

    #[structopt(long, possible_values = &Heap::variants(), case_insensitive = true, default_value = "lh")]
    pq: Heap,

    #[structopt(long, possible_values = &Map::variants(), case_insensitive = true, default_value = "tm")]
    map: Map,

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
    match (opt.tree, opt.pq, opt.map) {
        (Tree::Lbt, Heap::Ch, Map::Btm) => {
            let tree = HuffmanCodingTree::<LinkedBinaryTree<_>>::new::<
                CompleteMaxHeap<_>,
                BTreeMap<_, _>,
                BTreeMap<_, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Lbt, Heap::Ch, Map::Hm) => {
            let tree = HuffmanCodingTree::<LinkedBinaryTree<_>>::new::<
                CompleteMaxHeap<_>,
                HashMap<_, _>,
                HashMap<_, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Lbt, Heap::Ch, Map::Tm) => {
            let tree = HuffmanCodingTree::<LinkedBinaryTree<_>>::new::<
                CompleteMaxHeap<_>,
                TreeMap<LinkedBinaryTree<_>, _, _>,
                TreeMap<LinkedBinaryTree<_>, _, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Lbt, Heap::Ch, Map::Avl) => {
            let tree = HuffmanCodingTree::<LinkedBinaryTree<_>>::new::<
                CompleteMaxHeap<_>,
                AVLTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
                AVLTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Lbt, Heap::Ch, Map::Stm) => {
            let tree = HuffmanCodingTree::<LinkedBinaryTree<_>>::new::<
                CompleteMaxHeap<_>,
                SplayTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
                SplayTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Lbt, Heap::Ch, Map::Mbtm) => {
            let tree = HuffmanCodingTree::<LinkedBinaryTree<_>>::new::<
                CompleteMaxHeap<_>,
                MyBTreeMap<_, _, MBTM_ARG>,
                MyBTreeMap<_, _, MBTM_ARG>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Lbt, Heap::Ch, Map::Rbtm) => {
            let tree = HuffmanCodingTree::<LinkedBinaryTree<_>>::new::<
                CompleteMaxHeap<_>,
                RBTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
                RBTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Lbt, Heap::Lh, Map::Btm) => {
            let tree = HuffmanCodingTree::<LinkedBinaryTree<_>>::new::<
                LeftHeap<_>,
                BTreeMap<_, _>,
                BTreeMap<_, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Lbt, Heap::Lh, Map::Hm) => {
            let tree = HuffmanCodingTree::<LinkedBinaryTree<_>>::new::<
                LeftHeap<_>,
                HashMap<_, _>,
                HashMap<_, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Lbt, Heap::Lh, Map::Tm) => {
            let tree = HuffmanCodingTree::<LinkedBinaryTree<_>>::new::<
                LeftHeap<_>,
                TreeMap<LinkedBinaryTree<_>, _, _>,
                TreeMap<LinkedBinaryTree<_>, _, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Lbt, Heap::Lh, Map::Avl) => {
            let tree = HuffmanCodingTree::<LinkedBinaryTree<_>>::new::<
                LeftHeap<_>,
                AVLTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
                AVLTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Lbt, Heap::Lh, Map::Stm) => {
            let tree = HuffmanCodingTree::<LinkedBinaryTree<_>>::new::<
                LeftHeap<_>,
                SplayTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
                SplayTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Lbt, Heap::Lh, Map::Mbtm) => {
            let tree = HuffmanCodingTree::<LinkedBinaryTree<_>>::new::<
                LeftHeap<_>,
                MyBTreeMap<_, _, MBTM_ARG>,
                MyBTreeMap<_, _, MBTM_ARG>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Lbt, Heap::Lh, Map::Rbtm) => {
            let tree = HuffmanCodingTree::<LinkedBinaryTree<_>>::new::<
                LeftHeap<_>,
                RBTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
                RBTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Vbt, Heap::Ch, Map::Btm) => {
            let tree = HuffmanCodingTree::<VecBinaryTree<_>>::new::<
                CompleteMaxHeap<_>,
                BTreeMap<_, _>,
                BTreeMap<_, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Vbt, Heap::Ch, Map::Hm) => {
            let tree = HuffmanCodingTree::<VecBinaryTree<_>>::new::<
                CompleteMaxHeap<_>,
                HashMap<_, _>,
                HashMap<_, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Vbt, Heap::Ch, Map::Tm) => {
            let tree = HuffmanCodingTree::<VecBinaryTree<_>>::new::<
                CompleteMaxHeap<_>,
                TreeMap<LinkedBinaryTree<_>, _, _>,
                TreeMap<LinkedBinaryTree<_>, _, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Vbt, Heap::Ch, Map::Avl) => {
            let tree = HuffmanCodingTree::<VecBinaryTree<_>>::new::<
                CompleteMaxHeap<_>,
                AVLTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
                AVLTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Vbt, Heap::Ch, Map::Stm) => {
            let tree = HuffmanCodingTree::<VecBinaryTree<_>>::new::<
                CompleteMaxHeap<_>,
                SplayTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
                SplayTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Vbt, Heap::Ch, Map::Mbtm) => {
            let tree = HuffmanCodingTree::<VecBinaryTree<_>>::new::<
                CompleteMaxHeap<_>,
                MyBTreeMap<_, _, MBTM_ARG>,
                MyBTreeMap<_, _, MBTM_ARG>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Vbt, Heap::Ch, Map::Rbtm) => {
            let tree = HuffmanCodingTree::<VecBinaryTree<_>>::new::<
                CompleteMaxHeap<_>,
                RBTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
                RBTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Vbt, Heap::Lh, Map::Btm) => {
            let tree = HuffmanCodingTree::<VecBinaryTree<_>>::new::<
                LeftHeap<_>,
                BTreeMap<_, _>,
                BTreeMap<_, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Vbt, Heap::Lh, Map::Hm) => {
            let tree = HuffmanCodingTree::<VecBinaryTree<_>>::new::<
                LeftHeap<_>,
                HashMap<_, _>,
                HashMap<_, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Vbt, Heap::Lh, Map::Tm) => {
            let tree = HuffmanCodingTree::<VecBinaryTree<_>>::new::<
                LeftHeap<_>,
                TreeMap<LinkedBinaryTree<_>, _, _>,
                TreeMap<LinkedBinaryTree<_>, _, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Vbt, Heap::Lh, Map::Avl) => {
            let tree = HuffmanCodingTree::<VecBinaryTree<_>>::new::<
                LeftHeap<_>,
                AVLTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
                AVLTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Vbt, Heap::Lh, Map::Stm) => {
            let tree = HuffmanCodingTree::<VecBinaryTree<_>>::new::<
                LeftHeap<_>,
                SplayTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
                SplayTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Vbt, Heap::Lh, Map::Mbtm) => {
            let tree = HuffmanCodingTree::<VecBinaryTree<_>>::new::<
                LeftHeap<_>,
                MyBTreeMap<_, _, MBTM_ARG>,
                MyBTreeMap<_, _, MBTM_ARG>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Vbt, Heap::Lh, Map::Rbtm) => {
            let tree = HuffmanCodingTree::<VecBinaryTree<_>>::new::<
                LeftHeap<_>,
                RBTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
                RBTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Dlbt, Heap::Ch, Map::Btm) => {
            let tree = HuffmanCodingTree::<DoublyLinkedBinaryTree<_>>::new::<
                CompleteMaxHeap<_>,
                BTreeMap<_, _>,
                BTreeMap<_, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Dlbt, Heap::Ch, Map::Hm) => {
            let tree = HuffmanCodingTree::<DoublyLinkedBinaryTree<_>>::new::<
                CompleteMaxHeap<_>,
                HashMap<_, _>,
                HashMap<_, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Dlbt, Heap::Ch, Map::Tm) => {
            let tree = HuffmanCodingTree::<DoublyLinkedBinaryTree<_>>::new::<
                CompleteMaxHeap<_>,
                TreeMap<LinkedBinaryTree<_>, _, _>,
                TreeMap<LinkedBinaryTree<_>, _, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Dlbt, Heap::Ch, Map::Avl) => {
            let tree = HuffmanCodingTree::<DoublyLinkedBinaryTree<_>>::new::<
                CompleteMaxHeap<_>,
                AVLTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
                AVLTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Dlbt, Heap::Ch, Map::Stm) => {
            let tree = HuffmanCodingTree::<DoublyLinkedBinaryTree<_>>::new::<
                CompleteMaxHeap<_>,
                SplayTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
                SplayTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Dlbt, Heap::Ch, Map::Mbtm) => {
            let tree = HuffmanCodingTree::<DoublyLinkedBinaryTree<_>>::new::<
                CompleteMaxHeap<_>,
                MyBTreeMap<_, _, MBTM_ARG>,
                MyBTreeMap<_, _, MBTM_ARG>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Dlbt, Heap::Ch, Map::Rbtm) => {
            let tree = HuffmanCodingTree::<DoublyLinkedBinaryTree<_>>::new::<
                CompleteMaxHeap<_>,
                RBTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
                RBTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Dlbt, Heap::Lh, Map::Btm) => {
            let tree = HuffmanCodingTree::<DoublyLinkedBinaryTree<_>>::new::<
                LeftHeap<_>,
                BTreeMap<_, _>,
                BTreeMap<_, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Dlbt, Heap::Lh, Map::Hm) => {
            let tree = HuffmanCodingTree::<DoublyLinkedBinaryTree<_>>::new::<
                LeftHeap<_>,
                HashMap<_, _>,
                HashMap<_, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Dlbt, Heap::Lh, Map::Tm) => {
            let tree = HuffmanCodingTree::<DoublyLinkedBinaryTree<_>>::new::<
                LeftHeap<_>,
                TreeMap<LinkedBinaryTree<_>, _, _>,
                TreeMap<LinkedBinaryTree<_>, _, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Dlbt, Heap::Lh, Map::Avl) => {
            let tree = HuffmanCodingTree::<DoublyLinkedBinaryTree<_>>::new::<
                LeftHeap<_>,
                AVLTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
                AVLTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Dlbt, Heap::Lh, Map::Stm) => {
            let tree = HuffmanCodingTree::<DoublyLinkedBinaryTree<_>>::new::<
                LeftHeap<_>,
                SplayTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
                SplayTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Dlbt, Heap::Lh, Map::Mbtm) => {
            let tree = HuffmanCodingTree::<DoublyLinkedBinaryTree<_>>::new::<
                LeftHeap<_>,
                MyBTreeMap<_, _, MBTM_ARG>,
                MyBTreeMap<_, _, MBTM_ARG>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
        (Tree::Dlbt, Heap::Lh, Map::Rbtm) => {
            let tree = HuffmanCodingTree::<DoublyLinkedBinaryTree<_>>::new::<
                LeftHeap<_>,
                RBTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
                RBTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
            >(&contents)
            .unwrap();
            let (_, len) = tree.encoded();
            println!("len = {}bit", len);
            #[cfg(debug)]
            assert_eq!(tree.decode(), contents);
        }
    }
    Ok(())
}
