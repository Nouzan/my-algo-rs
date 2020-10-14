use super::vec_binary_tree::{cursor::Cursor, VecBinaryTree};
use super::*;
use crate::ch2::PartialOrdListExt;
use bitstream_io::{BigEndian, BitReader, BitWriter};
use std::cmp::{Ordering, PartialOrd};
use std::collections::BTreeMap;
use std::io;

pub fn char_count(text: &str) -> BTreeMap<char, usize> {
    let mut map = BTreeMap::new();
    for c in text.chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    map
}

/// 从编码树建立编码表.
/// # Panics
/// 要求树至少包含2个结点，叶子结点非空，且每个叶子存储的字符不同.
fn generate_encoding_map(tree: &VecBinaryTree<HuffmanChar>) -> BTreeMap<char, Vec<bool>> {
    let mut code = Vec::new();
    let mut left_stack = Vec::new();
    let mut right_stack = Vec::new();
    let mut map = BTreeMap::new();

    fn push_deep_most_chain<'a>(
        code: &mut Vec<bool>,
        left_stack: &mut Vec<Cursor<'a, HuffmanChar>>,
        right_stack: &mut Vec<Cursor<'a, HuffmanChar>>,
        current: &mut Cursor<'a, HuffmanChar>,
    ) {
        while !current.is_empty_subtree() {
            if current.left().is_some() {
                code.push(true);
                left_stack.push(current.clone());
                current.move_left();
            } else if current.right().is_some() {
                code.push(false);
                right_stack.push(current.clone());
                current.move_right();
            } else {
                right_stack.push(current.clone());
                break;
            }
        }
    };

    let mut root = tree.cursor();
    push_deep_most_chain(&mut code, &mut left_stack, &mut right_stack, &mut root);

    loop {
        if let Some(current) = right_stack.pop() {
            if current.is_leaf() {
                map.insert((*current.as_ref().unwrap()).ch.unwrap(), code.clone());
            }
            code.pop();
        } else if let Some(mut current) = left_stack.pop() {
            right_stack.push(current.clone());
            current.move_right();
            code.push(false);
            push_deep_most_chain(&mut code, &mut left_stack, &mut right_stack, &mut current)
        } else {
            break;
        }
    }

    map
}

struct HuffmanChar {
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
        self.count.partial_cmp(&other.count)
    }
}

pub struct HuffmanCodingTree {
    tree: VecBinaryTree<HuffmanChar>,
    encoded: Vec<u8>,
    len: usize,
}

impl PartialEq for VecBinaryTree<HuffmanChar> {
    fn eq(&self, other: &Self) -> bool {
        self.cursor().as_ref() == other.cursor().as_ref()
    }
}

impl PartialOrd for VecBinaryTree<HuffmanChar> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.cursor().as_ref(), other.cursor().as_ref()) {
            (Some(lc), Some(rc)) => lc.partial_cmp(rc),
            _ => None,
        }
    }
}

impl HuffmanCodingTree {
    pub fn new(text: &str) -> Option<Self> {
        let char_map = char_count(text);
        println!("{:?}", char_map);
        if char_map.is_empty() {
            None
        } else {
            // 创建编码森林
            let mut forest: Vec<_> = char_map
                .iter()
                .map(|(&ch, &count)| {
                    let mut tree = VecBinaryTree::new();
                    tree.cursor_mut()
                        .insert_as_root(HuffmanChar::new(Some(ch), count));
                    tree
                })
                .collect();

            // 自底向上建树
            while forest.len() > 1 {
                // TODO: use faster structure.
                let (mut lhs, mut rhs) =
                    (forest.delete_min().unwrap(), forest.delete_min().unwrap());
                let mut tree = VecBinaryTree::new();
                let mut cursor = tree.cursor_mut();
                let count =
                    lhs.cursor().as_ref().unwrap().count + rhs.cursor().as_ref().unwrap().count;
                cursor.insert_as_root(HuffmanChar::new(None, count));
                cursor.append_left(&mut lhs.cursor_mut());
                cursor.append_right(&mut rhs.cursor_mut());
                forest.push(tree);
            }
            let tree = forest.pop().unwrap();
            for ch in tree.cursor().pre_order_iter() {
                print!("{:?} ", ch.ch);
            }
            println!();
            for ch in tree.cursor().post_order_iter() {
                print!("{:?} ", ch.ch);
            }
            println!();

            // 建立编码表
            let encoding_map = generate_encoding_map(&tree);
            println!("{:?}", encoding_map);

            // 编码
            let (encoded, len) = Self::encode(text, &encoding_map);

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
    fn encode(text: &str, encoding_map: &BTreeMap<char, Vec<bool>>) -> (Vec<u8>, usize) {
        let mut writer = BitWriter::endian(Vec::new(), BigEndian);
        let mut len = 0;
        for ch in text.chars() {
            let code = encoding_map.get(&ch).unwrap();
            len += code.len();
            for &bit in code {
                writer.write_bit(bit).unwrap();
            }
        }
        writer.byte_align().unwrap();
        (writer.into_writer(), len)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encoding() {
        let s = String::from("hello, world!");
        let encoding_tree = HuffmanCodingTree::new(&s).unwrap();
        println!("{:?}", encoding_tree.encoded());
        println!("{}", encoding_tree.decode());
    }
}
