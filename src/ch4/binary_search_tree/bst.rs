use super::{Entry, Map};
use crate::ch4::{BinTreeCursor, BinTreeCursorMut, BinTreeMut};
use std::cmp::Ordering;
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
    fn delete_at(cursor: &mut Tree::CursorMut<'_>) -> V {
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

    /// 沿树下降搜索.
    ///
    /// 若`cursor`所指结点不存在，则返回`None`.
    ///
    /// 若命中，则返回`Some(Ordering::Equal)`，此时`cursor`指向命中的目标.
    ///
    /// 若没命中，则返回`Some(Ordering::Less)`或`Some(Ordering::Greater)`，分别表示命中原因，此时`cursor`指向目标位置的父母.
    fn move_to_target<'a, C>(cursor: &mut C, target: &K) -> Option<Ordering>
    where
        C: BinTreeCursor<'a, Elem = Tree::Elem>,
    {
        while let Some(entry) = cursor.as_ref() {
            match target.cmp(&entry.key) {
                Ordering::Equal => return Some(Ordering::Equal),
                Ordering::Less => {
                    if cursor.left().is_none() {
                        return Some(Ordering::Less);
                    } else {
                        cursor.move_left();
                    }
                }
                Ordering::Greater => {
                    if cursor.right().is_none() {
                        return Some(Ordering::Greater);
                    } else {
                        cursor.move_right();
                    }
                }
            }
        }
        None
    }
}

impl<K: Ord, V, Tree: Default + BinTreeMut<Elem = Entry<K, V>>> Map<K, V> for TreeMap<Tree, K, V> {
    fn get(&self, key: &K) -> Option<&V> {
        let mut cursor = self.tree.cursor();
        if let Some(Ordering::Equal) = Self::move_to_target(&mut cursor, key) {
            cursor.into_ref().map(|entry| &entry.value)
        } else {
            None
        }
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let mut cursor = self.tree.cursor_mut();
        if let Some(Ordering::Equal) = Self::move_to_target(&mut cursor, key) {
            cursor.into_mut().map(|entry| &mut entry.value)
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn insert(&mut self, key: K, mut value: V) -> Option<V> {
        let mut parent = self.tree.cursor_mut();
        match Self::move_to_target(&mut parent, &key) {
            Some(Ordering::Equal) => {
                mem::swap(&mut parent.into_mut().unwrap().value, &mut value);
                Some(value)
            }
            Some(Ordering::Less) => {
                parent.insert_as_left(Entry { key, value });
                self.len += 1;
                None
            }
            Some(Ordering::Greater) => {
                parent.insert_as_right(Entry { key, value });
                self.len += 1;
                None
            }
            None => {
                parent.insert_as_root(Entry { key, value });
                self.len += 1;
                None
            }
        }
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let mut cursor = self.tree.cursor_mut();
        if let Some(Ordering::Equal) = Self::move_to_target(&mut cursor, key) {
            self.len -= 1;
            Some(Self::delete_at(&mut cursor))
        } else {
            None
        }
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
            if let Some(elem) = map.get_mut(k) {
                *elem += 1
            }
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
                if let Some(elem) = map.get_mut(k) { *elem += 1 };
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
                if let Some(elem) = map.get_mut(k) { *elem += 1 }
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
