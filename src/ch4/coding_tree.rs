use super::vec_binary_tree::VecBinaryTree;
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
    let mut stack = Vec::new(); // 保存着已经左转、但还未右转的结点.
    let mut map = BTreeMap::new();
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

            // 建立编码表
            let encoding_map = generate_encoding_map(&tree);

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
        assert_eq!(s, encoding_tree.decode());
    }
}
