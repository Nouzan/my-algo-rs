use super::{bst::TreeMap, Entry, Map};
use crate::ch4::{BinTreeCursorMut, MoveParentBinTreeMut, MoveParentCursorMut};
use std::cmp::Ordering;
use std::mem;

/// 伸展树.
///
/// # Warnings
/// - 请不要尝试使用`VevBinaryTree`去实现`SplayTreeMap`.
pub struct SplayTreeMap<Tree: MoveParentBinTreeMut<Elem = Entry<K, V>>, K: Ord, V> {
    bst: TreeMap<Tree, K, V>,
}

impl<Tree: Default + MoveParentBinTreeMut<Elem = Entry<K, V>>, K: Ord, V> SplayTreeMap<Tree, K, V> {
    fn splay<'a, C>(cursor: &mut C)
    where
        C: MoveParentCursorMut<'a, Elem = Tree::Elem>
            + BinTreeCursorMut<'a, Elem = Tree::Elem, SubTree = Tree>,
    {
        if !cursor.is_empty_subtree() {
            while cursor.parent().is_some() {
                let (vl, vr) = (cursor.take_left().unwrap(), cursor.take_right().unwrap());
                let vflag = cursor.is_left_child();
                cursor.move_parent();
                let (mut v, pt) = if vflag {
                    (cursor.take_left().unwrap(), cursor.take_right().unwrap())
                } else {
                    (cursor.take_right().unwrap(), cursor.take_left().unwrap())
                };
                if cursor.parent().is_some() {
                    let pflag = cursor.is_left_child();
                    cursor.move_parent();
                    let (mut p, gt) = if pflag {
                        (cursor.take_left().unwrap(), cursor.take_right().unwrap())
                    } else {
                        (cursor.take_right().unwrap(), cursor.take_left().unwrap())
                    };
                    let mut g = cursor.take();
                    match (vflag, pflag) {
                        (true, true) => {
                            // vl [v] vr [p] pt [g] gt
                            g.cursor_mut().append_left(pt);
                            g.cursor_mut().append_right(gt);
                            p.cursor_mut().append_left(vr);
                            p.cursor_mut().append_right(g);
                            v.cursor_mut().append_left(vl);
                            v.cursor_mut().append_right(p);
                        }
                        (false, false) => {
                            // gt [g] pt [p] vl [v] vr
                            g.cursor_mut().append_left(gt);
                            g.cursor_mut().append_right(pt);
                            p.cursor_mut().append_left(g);
                            p.cursor_mut().append_right(vl);
                            v.cursor_mut().append_left(p);
                            v.cursor_mut().append_right(vr);
                        }
                        (true, false) => {
                            // gt [g] vl [v] vr [p] pt
                            g.cursor_mut().append_left(gt);
                            g.cursor_mut().append_right(vl);
                            p.cursor_mut().append_left(vr);
                            p.cursor_mut().append_right(pt);
                            v.cursor_mut().append_left(g);
                            v.cursor_mut().append_right(p);
                        }
                        (false, true) => {
                            // pt [p] vl [v] vr [g] gt
                            g.cursor_mut().append_left(vr);
                            g.cursor_mut().append_right(gt);
                            p.cursor_mut().append_left(pt);
                            p.cursor_mut().append_right(vl);
                            v.cursor_mut().append_left(p);
                            v.cursor_mut().append_right(g);
                        }
                    }
                } else if vflag {
                    // vl [v] vr [p] pt
                    let mut p = cursor.take();
                    p.cursor_mut().append_left(vr);
                    p.cursor_mut().append_right(pt);
                    v.cursor_mut().append_left(vl);
                    v.cursor_mut().append_right(p);
                } else {
                    // pt [p] vl [v] vr
                    let mut p = cursor.take();
                    p.cursor_mut().append_left(pt);
                    p.cursor_mut().append_right(vl);
                    v.cursor_mut().append_left(p);
                    v.cursor_mut().append_right(vr);
                }
                cursor.append(v);
            }
        }
    }

    fn move_to_target_and_splay<'a, C>(cursor: &mut C, target: &K) -> Option<Ordering>
    where
        C: MoveParentCursorMut<'a, Elem = Tree::Elem>
            + BinTreeCursorMut<'a, Elem = Tree::Elem, SubTree = Tree>,
    {
        let result = TreeMap::<Tree, K, V>::move_to_target(cursor, target);
        Self::splay(cursor);
        result
    }
}

impl<Tree: Default + MoveParentBinTreeMut<Elem = Entry<K, V>>, K: Ord, V> Default
    for SplayTreeMap<Tree, K, V>
{
    fn default() -> Self {
        Self {
            bst: TreeMap::default(),
        }
    }
}

impl<Tree: Default + MoveParentBinTreeMut<Elem = Entry<K, V>>, K: Ord, V> Map<K, V>
    for SplayTreeMap<Tree, K, V>
{
    /// 返回键所对应的值的引用.
    ///
    /// 这不会改变树结构，不会进行伸展操作.
    /// 所以为了更好的伸展树性能，请尽可能使用`get_mut`.
    fn get(&self, key: &K) -> Option<&V> {
        self.bst.get(key)
    }

    /// 返回键所对应的值的可变引用.
    ///
    /// 无论是否命中，都会将树中与`key`最接近的某个结点伸展至到根.
    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let mut cursor = self.bst.tree.move_parent_cursor_mut();
        if let Some(Ordering::Equal) = Self::move_to_target_and_splay(&mut cursor, key) {
            cursor.into_mut().map(|entry| &mut entry.value)
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.bst.len()
    }

    fn iter<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = (&K, &V)>> {
        self.bst.iter()
    }

    fn get_mut_or_insert(&mut self, key: K, default: V) -> &mut V {
        let mut parent = self.bst.tree.move_parent_cursor_mut();
        match Self::move_to_target_and_splay(&mut parent, &key) {
            Some(Ordering::Equal) => (),
            Some(Ordering::Less) => {
                // pl [v] [p] pr
                let pl = parent.take_left().unwrap();
                let p = parent.take();
                parent.insert_as_root(Entry {
                    key,
                    value: default,
                });
                parent.append_left(pl);
                parent.append_right(p);
                self.bst.len += 1;
            }
            Some(Ordering::Greater) => {
                // pl [p] [v] pr
                let pr = parent.take_right().unwrap();
                let p = parent.take();
                parent.insert_as_root(Entry {
                    key,
                    value: default,
                });
                parent.append_right(pr);
                parent.append_left(p);
                self.bst.len += 1;
            }
            None => {
                parent.insert_as_root(Entry {
                    key,
                    value: default,
                });
                self.bst.len += 1;
            }
        }
        &mut parent.into_mut().unwrap().value
    }

    fn insert(&mut self, key: K, mut value: V) -> Option<V> {
        let mut parent = self.bst.tree.move_parent_cursor_mut();
        match Self::move_to_target_and_splay(&mut parent, &key) {
            Some(Ordering::Equal) => {
                mem::swap(&mut parent.into_mut().unwrap().value, &mut value);
                Some(value)
            }
            Some(Ordering::Less) => {
                // pl [v] [p] pr
                let pl = parent.take_left().unwrap();
                let p = parent.take();
                parent.insert_as_root(Entry { key, value });
                parent.append_left(pl);
                parent.append_right(p);
                self.bst.len += 1;
                None
            }
            Some(Ordering::Greater) => {
                // pl [p] [v] pr
                let pr = parent.take_right().unwrap();
                let p = parent.take();
                parent.insert_as_root(Entry { key, value });
                parent.append_right(pr);
                parent.append_left(p);
                self.bst.len += 1;
                None
            }
            None => {
                parent.insert_as_root(Entry { key, value });
                self.bst.len += 1;
                None
            }
        }
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let mut cursor = self.bst.tree.move_parent_cursor_mut();
        if let Some(Ordering::Equal) = Self::move_to_target_and_splay(&mut cursor, key) {
            self.bst.len -= 1;
            let (lhs, mut rhs) = (cursor.take_left().unwrap(), cursor.take_right().unwrap());
            let mut v = cursor.take();
            let mut rc = rhs.move_parent_cursor_mut();
            if Self::move_to_target_and_splay(&mut rc, key).is_some() {
                // 查找必然失败，且`rc`的左子树必然为空.
                rc.append_left(lhs);
                drop(rc);
                cursor.append(rhs);
            } else {
                cursor.append(lhs);
            }
            v.cursor_mut().into_inner().map(|entry| entry.value)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ch4::doubly_linked_binary_tree::DoublyLinkedBinaryTree;
    // use crate::ch4::vec_binary_tree::VecBinaryTree;
    use crate::ch4::{BinTree, BinTreeCursor};
    use proptest::prelude::*;
    use std::collections::HashMap;

    // #[test]
    // fn test_map_basic() {
    //     let mut data = HashMap::new();
    //     data.insert("Hello", 1);
    //     data.insert("World", 2);
    //     data.insert("!", 3);
    //     data.insert("Good", 4);
    //     let mut map = SplayTreeMap::<VecBinaryTree<_>, _, _>::default();
    //     for (k, v) in data.clone() {
    //         assert!(map.insert(k, v).is_none());
    //     }
    //     for k in data.keys() {
    //         if let Some(elem) = map.get_mut(k) {
    //             *elem += 1
    //         }
    //         assert_eq!(map.get(k).copied(), data.get(k).map(|elem| elem + 1));
    //     }
    //     for k in data.keys().cloned() {
    //         let elem = data.get(&k).copied().unwrap();
    //         assert_eq!(map.insert(k, elem), Some(elem + 1));
    //     }
    //     println!("{:?}", map.bst.tree.inner);
    //     for k in data.keys() {
    //         println!("{}", k);
    //         assert_eq!(map.remove(k), data.get(k).copied());
    //         println!("{:?}", map.bst.tree.inner);
    //     }
    // }

    #[test]
    fn test_map_basic_dlbt() {
        let mut data = HashMap::new();
        data.insert("Hello", 1);
        data.insert("World", 2);
        data.insert("!", 3);
        data.insert("Good", 4);
        data.insert("Job", 5);
        data.insert("Hhaha", 6);
        data.insert("Xfwawd", 7);
        data.insert("Gooo", 8);
        data.insert("jiojoij", 9);
        let mut map = SplayTreeMap::<DoublyLinkedBinaryTree<_>, _, _>::default();
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
        // println!("{:?}", map.bst.tree.inner);
        for k in data.keys() {
            println!("{}", k);
            assert_eq!(map.remove(k), data.get(k).copied());
            // println!("{:?}", map.bst.tree.inner);
        }
    }

    proptest! {
        #[test]
        fn test_map_dlbt(mut data: HashMap<String, i64>, random: String) {
            let mut map = SplayTreeMap::<DoublyLinkedBinaryTree<_>, _, _>::default();
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
                // 伸展性
                assert_eq!(map.bst.tree.cursor().into_ref().map(|entry| &entry.key), Some(k))
            }

            assert_eq!(map.len(), data.len());

            // replace by insert
            for k in data.keys().cloned() {
                let elem = data.get(&k).copied().unwrap();
                assert_eq!(map.insert(k.clone(), elem), Some(elem + 1));
                // 伸展性
                assert_eq!(map.bst.tree.cursor().into_ref().map(|entry| &entry.key), Some(&k))
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
