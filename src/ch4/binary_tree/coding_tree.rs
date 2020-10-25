use super::super::PriorityQueue;
use super::*;
use crate::ch4::Map;
use crate::vec::MyVec;
use bitstream_io::{BigEndian, BitReader, BitWriter};
use std::cmp::{Ordering, PartialOrd};
use std::io;

pub fn char_count<M: Map<char, usize>>(text: &str) -> M {
    let mut map = M::default();
    for c in text.chars() {
        *map.get_mut_or_insert(c, 0) += 1;
    }
    map
}

pub struct HuffmanChar {
    ch: Option<char>,
    count: usize,
}

impl HuffmanChar {
    fn new(ch: Option<char>, count: usize) -> Self {
        Self { ch, count }
    }
}

impl PartialEq for HuffmanChar {
    fn eq(&self, other: &HuffmanChar) -> bool {
        self.count == other.count
    }
}

impl PartialOrd for HuffmanChar {
    fn partial_cmp(&self, other: &HuffmanChar) -> Option<Ordering> {
        self.count
            .partial_cmp(&other.count)
            .map(|ordering| match ordering {
                // 较小者优先级较大.
                Ordering::Greater => Ordering::Less,
                Ordering::Less => Ordering::Greater,
                Ordering::Equal => Ordering::Equal,
            })
    }
}

pub struct HuffmanCodingTree<Tree> {
    tree: Tree,
    encoded: Vec<u8>,
    len: usize,
}

impl<Tree: Default + BinTreeMut<Elem = HuffmanChar> + PartialOrd> HuffmanCodingTree<Tree> {
    /// 从编码树建立编码表.
    /// # Panics
    /// 要求树至少包含2个结点，叶子结点非空，且每个叶子存储的字符不同.
    fn generate_encoding_map<M: Map<char, Vec<bool>>>(tree: &Tree) -> M {
        let mut code = Vec::new();
        let mut stack = Vec::new(); // 保存着已经左转、但还未右转的结点.
        let mut map = M::default();
        let mut current = tree.cursor();
        if current.is_leaf() {
            panic!("树必须包含至少2个结点!");
        } else if current.left().is_some() {
            stack.push(current.clone());
            code.push(true);
            current.move_left();
        } else {
            stack.push(current.clone());
            code.push(false);
            current.move_right();
        }

        while !stack.is_empty() {
            if current.is_leaf() {
                let ch = current.as_ref().unwrap().ch.unwrap();
                map.insert(ch, code.clone());
                while let Some(back) = stack.pop() {
                    if code.pop().unwrap() && back.right().is_some() {
                        stack.push(back.clone());
                        code.push(false);
                        current = back;
                        current.move_right();
                        break;
                    }
                }
            } else if current.left().is_some() {
                stack.push(current.clone());
                code.push(true);
                current.move_left();
            } else {
                stack.push(current.clone());
                code.push(false);
                current.move_right();
            }
        }

        map
    }

    /// 创建Huffman编码树，并对`text`进行编码.
    /// # Panics
    /// `text`中不同的字符数必须大于`1`.
    pub fn new<Pq: PriorityQueue<Tree>, M1: Map<char, Vec<bool>>, M2: Map<char, usize>>(
        text: &str,
    ) -> Option<Self> {
        println!("Counting...");
        let char_map: M2 = char_count(text);
        if char_map.is_empty() {
            None
        } else {
            println!("Building...");
            // 创建编码森林
            let forest: Vec<_> = char_map
                .iter()
                .map(|(&ch, &count)| {
                    let mut tree = Tree::default();
                    tree.cursor_mut()
                        .insert_as_root(HuffmanChar::new(Some(ch), count));
                    tree
                })
                .collect();

            let mut forest = Pq::from(MyVec::from(forest));

            // 自底向上建树
            while forest.len() > 1 {
                let (lhs, rhs) = (forest.delete_max().unwrap(), forest.delete_max().unwrap());
                let mut tree = Tree::default();
                let mut cursor = tree.cursor_mut();
                let count =
                    lhs.cursor().as_ref().unwrap().count + rhs.cursor().as_ref().unwrap().count;
                cursor.insert_as_root(HuffmanChar::new(None, count));
                cursor.append_left(lhs);
                cursor.append_right(rhs);
                drop(cursor);
                forest.insert(tree);
            }
            let tree = forest.delete_max().unwrap();

            println!("Code generating...");
            // 建立编码表
            let mut encoding_map: M1 = Self::generate_encoding_map(&tree);

            println!("Encoding...");
            // 编码
            let (encoded, len) = Self::encode(text, &mut encoding_map);

            Some(Self { tree, encoded, len })
        }
    }

    pub fn encoded(&self) -> (&Vec<u8>, usize) {
        (&self.encoded, self.len)
    }

    pub fn decode(&self) -> String {
        let mut reader = BitReader::endian(io::Cursor::new(&self.encoded), BigEndian);
        let mut decoded = String::new();
        let mut cursor = self.tree.cursor();
        for _ in 0..self.len {
            if reader.read_bit().unwrap() {
                cursor.move_left();
            } else {
                cursor.move_right();
            }
            if cursor.is_leaf() {
                let ch = (*cursor.as_ref().unwrap()).ch.unwrap();
                decoded.push(ch);
                cursor = self.tree.cursor();
            }
        }
        decoded
    }

    /// 编码字符串.
    /// # Panics
    /// 要求`text`中的所有字符均已被编码(存储在`encoding_map`中)，否则报错.
    fn encode<M: Map<char, Vec<bool>>>(text: &str, encoding_map: &mut M) -> (Vec<u8>, usize) {
        let mut writer = BitWriter::endian(Vec::new(), BigEndian);
        let mut len = 0;
        for ch in text.chars() {
            let code = encoding_map.get_mut(&ch).unwrap();
            len += code.len();
            for &mut bit in code {
                writer.write_bit(bit).unwrap();
            }
        }
        writer.byte_align().unwrap();
        (writer.into_writer(), len)
    }
}

#[cfg(test)]
mod test {
    use super::super::super::priority_queue::complete_heap::CompleteMaxHeap;
    use super::super::super::priority_queue::left_heap::LeftHeap;
    use super::super::linked_binary_tree::LinkedBinaryTree;
    use super::super::vec_binary_tree::VecBinaryTree;
    use super::*;
    use crate::ch4::avlt::AVLTreeMap;
    use crate::ch4::bt::BTreeMap;
    use crate::ch4::doubly_linked_binary_tree::DoublyLinkedBinaryTree;
    use crate::ch4::rbt::RBTreeMap;
    use proptest::prelude::*;

    fn at_least_two_disctint_chars(s: &str) -> bool {
        let mut cache = None;
        for ch in s.chars() {
            if let Some(cached) = cache {
                if cached != ch {
                    return true;
                }
            } else {
                cache = Some(ch);
            }
        }
        false
    }

    #[test]
    fn test_encoding() {
        let s = String::from("0j 1ￚ\u{a81}A_2¡𐝀®a𝒻kB￼lCVD");
        let encoding_tree = HuffmanCodingTree::<LinkedBinaryTree<_>>::new::<
            CompleteMaxHeap<_>,
            RBTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
            RBTreeMap<DoublyLinkedBinaryTree<_>, _, _>,
        >(&s)
        .unwrap();
        println!("{:?}", encoding_tree.encoded());
        assert_eq!(s, encoding_tree.decode());
    }

    proptest! {
        #[test]
        fn test_encoding_with_lbt_ch(s: String) {
            if at_least_two_disctint_chars(&s) {
                let encoding_tree =
                    HuffmanCodingTree::<LinkedBinaryTree<_>>::new::<CompleteMaxHeap<_>, AVLTreeMap<DoublyLinkedBinaryTree<_>, _, _>, AVLTreeMap<DoublyLinkedBinaryTree<_>, _, _>,>(&s).unwrap();
                assert_eq!(s, encoding_tree.decode());
            }
        }

        #[test]
        fn test_encoding_with_vbt_lh(s: String) {
            if at_least_two_disctint_chars(&s) {
                let encoding_tree =
                    HuffmanCodingTree::<VecBinaryTree<_>>::new::<LeftHeap<_>, AVLTreeMap<DoublyLinkedBinaryTree<_>, _, _>, AVLTreeMap<DoublyLinkedBinaryTree<_>, _, _>,>(&s).unwrap();
                assert_eq!(s, encoding_tree.decode());
            }
        }

        #[test]
        fn test_encoding_with_dlbt_lh(s: String) {
            if at_least_two_disctint_chars(&s) {
                let encoding_tree =
                    HuffmanCodingTree::<DoublyLinkedBinaryTree<_>>::new::<LeftHeap<_>, AVLTreeMap<DoublyLinkedBinaryTree<_>, _, _>, AVLTreeMap<DoublyLinkedBinaryTree<_>, _, _>,>(&s).unwrap();
                assert_eq!(s, encoding_tree.decode());
            }
        }

        #[test]
        fn test_encoding_with_btm_lh(s: String) {
            if at_least_two_disctint_chars(&s) {
                let encoding_tree =
                    HuffmanCodingTree::<DoublyLinkedBinaryTree<_>>::new::<LeftHeap<_>, BTreeMap<_, _, 4>, BTreeMap<_, _, 4>,>(&s).unwrap();
                assert_eq!(s, encoding_tree.decode());
            }
        }

        #[test]
        fn test_encoding_with_rbtm_lh(s: String) {
            if at_least_two_disctint_chars(&s) {
                let encoding_tree =
                    HuffmanCodingTree::<DoublyLinkedBinaryTree<_>>::new::<LeftHeap<_>, RBTreeMap<DoublyLinkedBinaryTree<_>, _, _>, RBTreeMap<DoublyLinkedBinaryTree<_>, _, _>,>(&s).unwrap();
                assert_eq!(s, encoding_tree.decode());
            }
        }
    }
}
