/// 红黑树(只允许存在红左链)
pub mod rbt2;

/// 二叉查找树(非generic)
pub mod bst2;

/// 红黑树
pub mod rbt;

/// B树.
pub mod bt;

/// 伸展树(Splay Tree).
pub mod st;

/// AVL树.
pub mod avlt;

/// 基础二叉查找树.
pub mod bst;

use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

/// 词条结构，表示一个键值对.
#[derive(Debug)]
pub struct Entry<K: Ord, V> {
    pub key: K,
    pub value: V,
}

impl<K: Ord, V> PartialEq for Entry<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(&other.key)
    }
}

impl<K: Ord, V> PartialOrd for Entry<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

impl<K: Ord, V> Eq for Entry<K, V> {}

impl<K: Ord, V> Ord for Entry<K, V> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

/// 字典特质.
pub trait Map<K: Ord, V>: Default {
    /// 返回键所对应的值的引用.
    fn get(&self, key: &K) -> Option<&V>;

    /// 返回键所对应的值的可变引用.
    fn get_mut(&mut self, key: &K) -> Option<&mut V>;

    /// 插入一个键值对.
    ///
    /// 若键不存在，则返回`None`.
    ///
    /// 若键存在，则更新它的值，并将旧值返回.
    fn insert(&mut self, key: K, value: V) -> Option<V>;

    /// 返回键所对应值的可变引用. 若不存在则插入`default`后，再返回可变引用.
    fn get_mut_or_insert(&mut self, key: K, default: V) -> &mut V;

    /// 移除一个键值对，并返回它的值.
    fn remove(&mut self, key: &K) -> Option<V>;

    /// 返回字典的大小.
    fn len(&self) -> usize;

    /// 字典是否为空.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn iter<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = (&K, &V)>>;
}

impl<K: Ord, V> Map<K, V> for BTreeMap<K, V> {
    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.get_mut(key)
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.insert(key, value)
    }

    fn get_mut_or_insert(&mut self, key: K, default: V) -> &mut V {
        self.entry(key).or_insert(default)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove(key)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn iter<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = (&K, &V)>> {
        Box::new(self.iter())
    }
}

impl<K: Hash + Ord, V> Map<K, V> for HashMap<K, V> {
    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.get_mut(key)
    }

    fn get_mut_or_insert(&mut self, key: K, default: V) -> &mut V {
        self.entry(key).or_insert(default)
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.insert(key, value)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove(key)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn iter<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = (&K, &V)>> {
        Box::new(self.iter())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ch4::{
        doubly_linked_binary_tree::DoublyLinkedBinaryTree, linked_binary_tree::LinkedBinaryTree,
    };
    use ::test::Bencher;
    use avlt::AVLTreeMap;
    use bst::TreeMap;
    use random::Source;
    use st::SplayTreeMap;
    use std::collections::{BTreeMap, HashMap};

    const N: usize = 10000;
    const M: usize = 100;
    const SEEDS: [u64; 2] = [42, 69];

    #[bench]
    fn bench_std_hm_random_insert(b: &mut Bencher) {
        let mut map = HashMap::<_, _>::default();
        let mut source = random::default().seed(SEEDS);
        for idx in source.iter::<i64>().take(N) {
            map.insert(idx, idx);
        }
        b.iter(|| {
            map.insert(source.read(), source.read());
        })
    }

    #[bench]
    fn bench_std_btm_random_insert(b: &mut Bencher) {
        let mut map = BTreeMap::<_, _>::default();
        let mut source = random::default().seed(SEEDS);
        for idx in source.iter::<i64>().take(N) {
            map.insert(idx, idx);
        }
        b.iter(|| {
            map.insert(source.read(), source.read());
        })
    }

    #[bench]
    fn bench_tm_random_insert(b: &mut Bencher) {
        let mut map = TreeMap::<LinkedBinaryTree<_>, _, _>::default();
        let mut source = random::default().seed(SEEDS);
        for idx in source.iter::<i64>().take(N) {
            map.insert(idx, idx);
        }
        b.iter(|| {
            map.insert(source.read(), source.read());
        })
    }

    #[bench]
    fn bench_stm_random_insert(b: &mut Bencher) {
        let mut map = SplayTreeMap::<DoublyLinkedBinaryTree<_>, _, _>::default();
        let mut source = random::default().seed(SEEDS);
        for idx in source.iter::<i64>().take(N) {
            map.insert(idx, idx);
        }
        b.iter(|| {
            map.insert(source.read(), source.read());
        })
    }

    #[bench]
    fn bench_avl_random_insert(b: &mut Bencher) {
        let mut map = AVLTreeMap::<DoublyLinkedBinaryTree<_>, _, _>::default();
        let mut source = random::default().seed(SEEDS);
        for idx in source.iter::<i64>().take(N) {
            map.insert(idx, idx);
        }
        b.iter(|| {
            map.insert(source.read(), source.read());
        })
    }

    #[bench]
    fn bench_std_hm_random_get(b: &mut Bencher) {
        let mut map = HashMap::<_, _>::default();
        let mut source = random::default().seed(SEEDS);
        for idx in source.iter::<i64>().take(N) {
            map.insert(idx, idx);
        }
        b.iter(|| {
            map.get(&source.read());
        })
    }

    #[bench]
    fn bench_std_btm_random_get(b: &mut Bencher) {
        let mut map = BTreeMap::<_, _>::default();
        let mut source = random::default().seed(SEEDS);
        for idx in source.iter::<i64>().take(N) {
            map.insert(idx, idx);
        }
        b.iter(|| {
            map.get(&source.read());
        })
    }

    #[bench]
    fn bench_tm_random_get(b: &mut Bencher) {
        let mut map = TreeMap::<LinkedBinaryTree<_>, _, _>::default();
        let mut source = random::default().seed(SEEDS);
        for idx in source.iter::<i64>().take(N) {
            map.insert(idx, idx);
        }
        b.iter(|| {
            map.get(&source.read());
        })
    }

    #[bench]
    fn bench_stm_random_get(b: &mut Bencher) {
        let mut map = SplayTreeMap::<DoublyLinkedBinaryTree<_>, _, _>::default();
        let mut source = random::default().seed(SEEDS);
        for idx in source.iter::<i64>().take(N) {
            map.insert(idx, idx);
        }
        b.iter(|| {
            map.get_mut(&source.read());
        })
    }

    #[bench]
    fn bench_avl_random_get(b: &mut Bencher) {
        let mut map = AVLTreeMap::<DoublyLinkedBinaryTree<_>, _, _>::default();
        let mut source = random::default().seed(SEEDS);
        for idx in source.iter::<i64>().take(N) {
            map.insert(idx, idx);
        }
        b.iter(|| {
            map.get(&source.read());
        })
    }

    #[bench]
    fn bench_std_hm_sequential_get(b: &mut Bencher) {
        let mut map = HashMap::<_, _>::default();
        let mut source = random::default().seed(SEEDS);
        for idx in source.iter::<i64>().take(N) {
            map.insert(idx, idx);
        }
        let mut gets = (0..M).map(|n| n as i64).cycle();
        b.iter(|| {
            map.get(&gets.next().unwrap());
        })
    }

    #[bench]
    fn bench_std_btm_sequential_get(b: &mut Bencher) {
        let mut map = BTreeMap::<_, _>::default();
        let mut source = random::default().seed(SEEDS);
        for idx in source.iter::<i64>().take(N) {
            map.insert(idx, idx);
        }
        let mut gets = (0..M).map(|n| n as i64).cycle();
        b.iter(|| {
            map.get(&gets.next().unwrap());
        })
    }

    #[bench]
    fn bench_tm_sequential_get(b: &mut Bencher) {
        let mut map = TreeMap::<LinkedBinaryTree<_>, _, _>::default();
        let mut source = random::default().seed(SEEDS);
        for idx in source.iter::<i64>().take(N) {
            map.insert(idx, idx);
        }
        let mut gets = (0..M).map(|n| n as i64).cycle();
        b.iter(|| {
            map.get(&gets.next().unwrap());
        })
    }

    #[bench]
    fn bench_stm_sequential_get(b: &mut Bencher) {
        let mut map = SplayTreeMap::<DoublyLinkedBinaryTree<_>, _, _>::default();
        let mut source = random::default().seed(SEEDS);
        for idx in source.iter::<i64>().take(N) {
            map.insert(idx, idx);
        }
        let mut gets = (0..M).map(|n| n as i64).cycle();
        b.iter(|| {
            map.get(&gets.next().unwrap());
        })
    }

    #[bench]
    fn bench_avl_sequential_get(b: &mut Bencher) {
        let mut map = AVLTreeMap::<DoublyLinkedBinaryTree<_>, _, _>::default();
        let mut source = random::default().seed(SEEDS);
        for idx in source.iter::<i64>().take(N) {
            map.insert(idx, idx);
        }
        let mut gets = (0..M).map(|n| n as i64).cycle();
        b.iter(|| {
            map.get(&gets.next().unwrap());
        })
    }
}
