use super::bst::TreeMap;
use super::{Entry, Map};
use crate::ch4::{
    BinTree, BinTreeCursor, BinTreeCursorMut, BinTreeMut, MoveParentBinTreeMut, MoveParentCursor,
};
use std::cmp::Ordering;
use std::mem;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct RBNode<T> {
    elem: T,
    is_black: bool,
}

impl<T> Deref for RBNode<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.elem
    }
}

impl<T> DerefMut for RBNode<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elem
    }
}

pub type RBEntry<K, V> = Entry<K, RBNode<V>>;

/// 基于红黑树的字典(Map).
pub struct RBTreeMap<Tree: BinTreeMut<Elem = RBEntry<K, V>>, K: Ord, V> {
    bst: TreeMap<Tree, K, RBNode<V>>,
}

impl<Tree, K: Ord, V> Default for RBTreeMap<Tree, K, V>
where
    Tree: Default + BinTreeMut<Elem = RBEntry<K, V>>,
{
    fn default() -> Self {
        Self {
            bst: TreeMap::default(),
        }
    }
}

impl<Tree, K: Ord, V> RBTreeMap<Tree, K, V>
where
    Tree: Default + BinTreeMut<Elem = RBEntry<K, V>>,
{
    pub fn new() -> Self {
        Self::default()
    }

    fn is_black<'a, C>(cursor: &C) -> bool
    where
        C: BinTreeCursor<'a, Elem = Tree::Elem>,
    {
        cursor.as_ref().map_or(true, |entry| entry.value.is_black)
    }

    fn set_black<'a, C>(cursor: &mut C)
    where
        C: BinTreeCursorMut<'a, Elem = Tree::Elem>,
    {
        if let Some(entry) = cursor.as_mut() {
            entry.value.is_black = true;
        }
    }

    fn set_red<'a, C>(cursor: &mut C)
    where
        C: BinTreeCursorMut<'a, Elem = Tree::Elem>,
    {
        if let Some(entry) = cursor.as_mut() {
            entry.value.is_black = false;
        }
    }

    /// 修复双红错误. 若最后`cursor`指向调用点，则返回`None`；否则`cursor`保持拓扑调整后的指向，并返回所进行的调整.
    fn solve_double_red<'a, C>(cursor: &mut C, v_flag: bool) -> Option<(bool, bool)>
    where
        C: MoveParentCursor<'a, Elem = Tree::Elem>
            + BinTreeCursorMut<'a, Elem = Tree::Elem, SubTree = Tree>,
        Tree: MoveParentBinTreeMut,
    {
        // cursor是红的，因此父母结点必然存在.
        let p_flag = cursor.is_left_child();
        let res = {
            // 找到v的叔父结点，判断属于哪种双红情况
            let mut uncle = cursor.move_parent_cursor();
            uncle.move_parent();
            if p_flag {
                uncle.move_right()
            } else {
                uncle.move_left()
            }
            Self::is_black(&uncle)
        };

        if res {
            // RR-1: 黑叔父
            match (p_flag, v_flag) {
                (true, true) => {
                    Self::set_black(cursor);
                    cursor.move_parent();
                    Self::set_red(cursor);
                    cursor.zig();
                    Some((true, true))
                }
                (false, false) => {
                    Self::set_black(cursor);
                    cursor.move_parent();
                    Self::set_red(cursor);
                    cursor.zag();
                    Some((false, false))
                }
                (true, false) => {
                    cursor.zag();
                    Self::set_black(cursor);
                    cursor.move_parent();
                    Self::set_red(cursor);
                    cursor.zig();
                    Some((true, false))
                }
                (false, true) => {
                    cursor.zig();
                    Self::set_black(cursor);
                    cursor.move_parent();
                    Self::set_red(cursor);
                    cursor.zag();
                    Some((false, true))
                }
            }
        } else {
            // RR-2: 红叔父
            cursor.move_parent();
            cursor.left_mut().unwrap().value.is_black = true;
            cursor.right_mut().unwrap().value.is_black = true;
            if cursor.parent().is_some() {
                Self::set_red(cursor);
                let is_left = cursor.is_left_child();
                cursor.move_parent();
                if !Self::is_black(cursor) {
                    if let Some((pz, vz)) = Self::solve_double_red(cursor, is_left) {
                        match (pz, vz) {
                            (true, true) | (false, false) => (),
                            (true, false) | (false, true) => {
                                if p_flag {
                                    cursor.move_left();
                                    cursor.move_right();
                                } else {
                                    cursor.move_right();
                                    cursor.move_left();
                                }
                            }
                        }
                    } else if is_left {
                        cursor.move_left()
                    } else {
                        cursor.move_right()
                    }
                } else if is_left {
                    cursor.move_left()
                } else {
                    cursor.move_right()
                }
            } else if p_flag {
                cursor.move_left()
            } else {
                cursor.move_right()
            }
            None
        }
    }

    fn solve_double_black<'a, C>(cursor: &mut C)
    where
        C: MoveParentCursor<'a, Elem = Tree::Elem>
            + BinTreeCursorMut<'a, Elem = Tree::Elem, SubTree = Tree>,
        Tree: MoveParentBinTreeMut,
    {
        let is_left = cursor.is_left_child();
        let mut poineer = cursor.move_parent_cursor();

        // 寻找兄弟结点(必然非外部结点).
        poineer.move_parent();
        let p_flag = if is_left {
            poineer.move_right();
            false
        } else {
            poineer.move_left();
            true
        };
        let p_black = Self::is_black(&poineer);
        let mut lp = poineer.cursor();
        lp.move_left();
        let mut rp = poineer.cursor();
        rp.move_right();
        let v_red = !Self::is_black(&lp) || !Self::is_black(&rp);
        let v_flag = !Self::is_black(&lp);
        drop(lp);
        drop(rp);
        drop(poineer);
        if p_black && v_red {
            // BB-1
            cursor.move_parent();
            let g_black = Self::is_black(cursor);
            Self::set_black(cursor);
            match (p_flag, v_flag) {
                (true, true) => {
                    cursor.zig();
                    if g_black {
                        Self::set_black(cursor)
                    } else {
                        Self::set_red(cursor)
                    }
                    cursor.move_left();
                    Self::set_black(cursor);
                }
                (false, false) => {
                    cursor.zag();
                    if g_black {
                        Self::set_black(cursor)
                    } else {
                        Self::set_red(cursor)
                    }
                    cursor.move_right();
                    Self::set_black(cursor);
                }
                (true, false) => {
                    if p_flag {
                        cursor.move_left()
                    } else {
                        cursor.move_right()
                    }
                    Self::set_black(cursor);
                    cursor.zag();
                    if g_black {
                        Self::set_black(cursor)
                    } else {
                        Self::set_red(cursor)
                    }
                    cursor.move_parent();
                    cursor.zig();
                }
                (false, true) => {
                    if p_flag {
                        cursor.move_left()
                    } else {
                        cursor.move_right()
                    }
                    Self::set_black(cursor);
                    cursor.zig();
                    if g_black {
                        Self::set_black(cursor)
                    } else {
                        Self::set_red(cursor)
                    }
                    cursor.move_parent();
                    cursor.zag();
                }
            }
        } else if p_black && !v_red {
            cursor.move_parent();
            if Self::is_black(cursor) {
                // BB-3
                if p_flag {
                    cursor.move_left()
                } else {
                    cursor.move_right()
                }
                Self::set_red(cursor);
                cursor.move_parent();
                if cursor.parent().is_some() {
                    Self::solve_double_black(cursor);
                }
            } else {
                // BB-2
                Self::set_black(cursor);
                if p_flag {
                    cursor.move_left()
                } else {
                    cursor.move_right()
                }
                Self::set_red(cursor);
            }
        } else {
            // BB-4
            cursor.move_parent();
            Self::set_red(cursor);
            if p_flag {
                cursor.zig()
            } else {
                cursor.zag()
            }
            Self::set_black(cursor);
            if p_flag {
                cursor.move_right();
                cursor.move_right();
            } else {
                cursor.move_left();
                cursor.move_left();
            }
            Self::solve_double_black(cursor);
        }
    }
}

impl<Tree, K: Ord, V> Map<K, V> for RBTreeMap<Tree, K, V>
where
    Tree: Default + MoveParentBinTreeMut<Elem = RBEntry<K, V>> + BinTreeMut<Elem = RBEntry<K, V>>,
{
    fn get(&self, key: &K) -> Option<&V> {
        self.bst.get(key).map(|node| &**node)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.bst.get_mut(key).map(|node| &mut **node)
    }

    fn len(&self) -> usize {
        self.bst.len()
    }

    fn get_mut_or_insert(&mut self, key: K, default: V) -> &mut V {
        let mut parent = self.bst.tree.move_parent_cursor_mut();
        let mut value = RBNode {
            elem: default,
            is_black: false,
        };
        let is_left = match TreeMap::<Tree, _, _>::move_to_target(&mut parent, &key) {
            Some(Ordering::Equal) => {
                return &mut parent.into_mut().unwrap().value.elem;
            }
            Some(Ordering::Less) => {
                parent.insert_as_left(Entry { key, value });
                self.bst.len += 1;
                true
            }
            Some(Ordering::Greater) => {
                parent.insert_as_right(Entry { key, value });
                self.bst.len += 1;
                false
            }
            None => {
                value.is_black = true;
                parent.insert_as_root(Entry { key, value });
                self.bst.len += 1;
                return &mut parent.into_mut().unwrap().value.elem;
            }
        };
        if !Self::is_black(&parent) {
            if let Some((pz, vz)) = Self::solve_double_red(&mut parent, is_left) {
                match (pz, vz) {
                    (true, true) | (false, false) => (),
                    (true, false) | (false, true) => {
                        return &mut parent.into_mut().unwrap().value.elem;
                    }
                }
            }
        }
        if is_left {
            parent.move_left()
        } else {
            parent.move_right()
        }
        &mut parent.into_mut().unwrap().value.elem
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut parent = self.bst.tree.move_parent_cursor_mut();
        let mut value = RBNode {
            elem: value,
            is_black: false,
        };
        let is_left = match TreeMap::<Tree, _, _>::move_to_target(&mut parent, &key) {
            Some(Ordering::Equal) => {
                value.is_black = Self::is_black(&parent);
                mem::swap(&mut parent.into_mut().unwrap().value, &mut value);
                return Some(value.elem);
            }
            Some(Ordering::Less) => {
                parent.insert_as_left(Entry { key, value });
                self.bst.len += 1;
                true
            }
            Some(Ordering::Greater) => {
                parent.insert_as_right(Entry { key, value });
                self.bst.len += 1;
                false
            }
            None => {
                value.is_black = true;
                parent.insert_as_root(Entry { key, value });
                self.bst.len += 1;
                return None;
            }
        };
        if !Self::is_black(&parent) {
            Self::solve_double_red(&mut parent, is_left);
        }
        None
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let mut cursor = self.bst.tree.move_parent_cursor_mut();
        if let Some(Ordering::Equal) = TreeMap::<Tree, _, _>::move_to_target(&mut cursor, key) {
            self.bst.len -= 1;
            let node = TreeMap::<Tree, _, _>::delete_at(&mut cursor, |x, y| {
                let Entry {
                    key: x_key,
                    value: x_node,
                } = x;
                let Entry {
                    key: y_key,
                    value: y_node,
                } = y;
                let x_value = &mut x_node.elem;
                let y_value = &mut y_node.elem;
                mem::swap(x_key, y_key);
                mem::swap(x_value, y_value);
            });
            if cursor.parent().is_none() {
                Self::set_black(&mut cursor);
            } else if node.is_black {
                if Self::is_black(&cursor) {
                    Self::solve_double_black(&mut cursor);
                } else {
                    Self::set_black(&mut cursor);
                }
            }
            Some(node.elem)
        } else {
            None
        }
    }

    fn iter<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = (&K, &V)>> {
        Box::new(self.bst.iter().map(|(k, node)| (k, &node.elem)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ch4::doubly_linked_binary_tree::DoublyLinkedBinaryTree;
    use proptest::prelude::*;
    use std::collections::HashMap;

    #[test]
    fn test_map_basic_dlbt() {
        let mut data: HashMap<i64, _> = HashMap::new();
        data.insert(-2776177022119650230, 1);
        data.insert(1620002456352265080, 2);
        data.insert(5331687563272558357, 3);
        data.insert(-8724383244456229708, 4);
        data.insert(44344689177037057, 5);
        data.insert(-2938427356902123924, 6);
        data.insert(-6897013407289520537, 7);
        // data.insert("Gooo", 8);
        // data.insert("jiojoij", 9);
        let random = -5886505072135295777;
        let mut map = RBTreeMap::<DoublyLinkedBinaryTree<_>, _, _>::default();
        assert!(map.is_empty());
        // insert
        for (k, v) in data.clone() {
            println!("{}", k);
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
            if let Some(elem) = map.get_mut(k) {
                *elem += 1
            };
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

    proptest! {
        #[test]
        fn test_map_dlbt(mut data: HashMap<String, i64>, random: String) {
            let mut map = RBTreeMap::<DoublyLinkedBinaryTree<_>, _, _>::default();
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
    }
}
