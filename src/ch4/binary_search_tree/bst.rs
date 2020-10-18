use super::{Entry, Map};
use crate::ch4::{BinTreeCursor, BinTreeCursorMut, BinTreeMut};
use std::mem;

/// 二叉查找树.
pub struct TreeMap<Tree: BinTreeMut<Elem = Entry<K, V>>, K: Ord, V> {
    tree: Tree,
    len: usize,
}

impl<K: Ord, V, Tree: Default + BinTreeMut<Elem = Entry<K, V>>> Default for TreeMap<Tree, K, V> {
    fn default() -> Self {
        Self {
            tree: Tree::default(),
            len: 0,
        }
    }
}

impl<K: Ord, V, Tree: Default + BinTreeMut<Elem = Entry<K, V>>> TreeMap<Tree, K, V> {
    /// 删除游标所指结点，并返回其值.
    /// # Panics
    /// `cursor`所指结点必须存在.
    fn delete_at(mut cursor: Tree::CursorMut<'_>) -> V {
        if cursor.left().is_none() {
            let tree = cursor.take_right().unwrap();
            let entry = cursor.take().cursor_mut().into_inner().unwrap();
            cursor.append(tree);
            entry.value
        } else if cursor.right().is_none() {
            let tree = cursor.take_left().unwrap();
            let entry = cursor.take().cursor_mut().into_inner().unwrap();
            cursor.append(tree);
            entry.value
        } else {
            let (current, succ) = cursor.move_succ_and_split_mut();
            let current = current.unwrap();
            let succ = succ.unwrap();
            mem::swap(current, succ);
            Self::delete_at(cursor)
        }
    }
}

impl<K: Ord, V, Tree: Default + BinTreeMut<Elem = Entry<K, V>>> Map<K, V> for TreeMap<Tree, K, V> {
    fn get(&self, key: &K) -> Option<&V> {
        let mut cursor = self.tree.cursor();
        while let Some(entry) = cursor.as_ref() {
            if *key == entry.key {
                return cursor.into_ref().map(|entry| &entry.value);
            } else if *key < entry.key {
                cursor.move_left();
            } else {
                cursor.move_right();
            }
        }
        None
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let mut cursor = self.tree.cursor_mut();
        while let Some(entry) = cursor.as_ref() {
            if *key == entry.key {
                return cursor.into_mut().map(|entry| &mut entry.value);
            } else if *key < entry.key {
                cursor.move_left();
            } else {
                cursor.move_right();
            }
        }
        None
    }

    fn len(&self) -> usize {
        self.len
    }

    fn insert(&mut self, key: K, mut value: V) -> Option<V> {
        let mut parent = self.tree.cursor_mut();
        while let Some(entry) = parent.as_ref() {
            if key == entry.key {
                mem::swap(&mut parent.into_mut().unwrap().value, &mut value);
                return Some(value);
            } else if key < entry.key {
                if parent.left().is_none() {
                    parent.insert_as_left(Entry { key, value });
                    self.len += 1;
                    return None;
                } else {
                    parent.move_left();
                }
            } else if key > entry.key {
                if parent.right().is_none() {
                    parent.insert_as_right(Entry { key, value });
                    self.len += 1;
                    return None;
                } else {
                    parent.move_right();
                }
            }
        }
        parent.insert_as_root(Entry { key, value });
        self.len += 1;
        None
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let mut cursor = self.tree.cursor_mut();
        while let Some(entry) = cursor.as_ref() {
            if *key == entry.key {
                self.len -= 1;
                return Some(Self::delete_at(cursor));
            } else if *key < entry.key {
                cursor.move_left();
            } else {
                cursor.move_right();
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ch4::linked_binary_tree::LinkedBinaryTree;
    use crate::ch4::vec_binary_tree::VecBinaryTree;
    use proptest::prelude::*;
    use std::collections::HashMap;

    #[test]
    fn test_map_basic() {
        let mut data = HashMap::new();
        data.insert("Hello", 1);
        data.insert("World", 2);
        data.insert("!", 3);
        data.insert("Good", 4);
        data.insert("Job", 5);
        let mut map = TreeMap::<VecBinaryTree<_>, _, _>::default();
        for (k, v) in data.clone() {
            assert!(map.insert(k, v).is_none());
        }
        for k in data.keys() {
            map.get_mut(k).map(|elem| *elem += 1);
            assert_eq!(map.get(k).copied(), data.get(k).map(|elem| elem + 1));
        }
        for k in data.keys().cloned() {
            let elem = data.get(&k).copied().unwrap();
            assert_eq!(map.insert(k, elem), Some(elem + 1));
        }
        for k in data.keys() {
            assert_eq!(map.remove(k), data.get(k).copied())
        }
    }

    proptest! {
        #[test]
        fn test_map_basic_vbt(mut data: HashMap<String, i64>, random: String) {
            let mut map = TreeMap::<VecBinaryTree<_>, _, _>::default();
            assert!(map.is_empty());
            // insert
            for (k, v) in data.clone() {
                assert!(map.insert(k, v).is_none());
            }

            assert_eq!(map.len(), data.len());

            // get
            for k in data.keys() {
                assert_eq!(map.get(k), data.get(k));
            }

            // random get
            assert_eq!(map.get(&random), data.get(&random));

            // get_mut
            for k in data.keys() {
                map.get_mut(k).map(|elem| *elem += 1);
                assert_eq!(map.get(k).copied(), data.get(k).map(|elem| elem + 1));
            }

            assert_eq!(map.len(), data.len());

            // replace by insert
            for k in data.keys().cloned() {
                let elem = data.get(&k).copied().unwrap();
                assert_eq!(map.insert(k, elem), Some(elem + 1));
            }

            assert_eq!(map.len(), data.len());

            // remove
            for (idx, k) in data.keys().enumerate() {
                assert_eq!(map.remove(k), data.get(k).copied());
                assert_eq!(map.len(), data.len() - idx - 1);
            }
            assert!(map.is_empty());
        }

        #[test]
        fn test_map_basic_lbt(mut data: HashMap<String, i64>, random: String) {
            let mut map = TreeMap::<LinkedBinaryTree<_>, _, _>::default();
            assert!(map.is_empty());
            // insert
            for (k, v) in data.clone() {
                assert!(map.insert(k, v).is_none());
            }

            assert_eq!(map.len(), data.len());

            // get
            for k in data.keys() {
                assert_eq!(map.get(k), data.get(k));
            }

            // random get
            assert_eq!(map.get(&random), data.get(&random));

            // get_mut
            for k in data.keys() {
                map.get_mut(k).map(|elem| *elem += 1);
                assert_eq!(map.get(k).copied(), data.get(k).map(|elem| elem + 1));
            }

            assert_eq!(map.len(), data.len());

            // replace by insert
            for k in data.keys().cloned() {
                let elem = data.get(&k).copied().unwrap();
                assert_eq!(map.insert(k, elem), Some(elem + 1));
            }

            assert_eq!(map.len(), data.len());

            // remove
            for (idx, k) in data.keys().enumerate() {
                assert_eq!(map.remove(k), data.get(k).copied());
                assert_eq!(map.len(), data.len() - idx - 1);
            }
            assert!(map.is_empty());
        }
    }
}
