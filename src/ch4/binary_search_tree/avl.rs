use super::bst::TreeMap;
use super::{Entry, Map};
use crate::ch4::{
    BinTreeCursor, BinTreeCursorMut, BinTreeMut, MoveParentBinTreeMut, MoveParentCursor,
};
use std::cmp::Ordering;
use std::mem;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct AVLNode<T> {
    elem: T,
    height: isize,
}

impl<T> Deref for AVLNode<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.elem
    }
}

impl<T> DerefMut for AVLNode<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elem
    }
}

pub struct AVLTreeMap<Tree: BinTreeMut<Elem = Entry<K, AVLNode<V>>>, K: Ord, V> {
    bst: TreeMap<Tree, K, AVLNode<V>>,
}

impl<Tree, K: Ord, V> Default for AVLTreeMap<Tree, K, V>
where
    Tree: Default + BinTreeMut<Elem = Entry<K, AVLNode<V>>>,
{
    fn default() -> Self {
        Self {
            bst: TreeMap::default(),
        }
    }
}

impl<Tree, K: Ord, V> AVLTreeMap<Tree, K, V>
where
    Tree: Default + BinTreeMut<Elem = Entry<K, AVLNode<V>>>,
{
    fn balance_factor<'a, C>(cursor: &C) -> isize
    where
        C: BinTreeCursor<'a, Elem = Tree::Elem>,
    {
        let lhs = cursor.left().map_or(-1, |node| node.value.height);
        let rhs = cursor.right().map_or(-1, |node| node.value.height);
        lhs - rhs
    }

    fn is_avl_balanced<'a, C>(cursor: &C) -> bool
    where
        C: BinTreeCursor<'a, Elem = Tree::Elem>,
    {
        let bf = Self::balance_factor(cursor);
        -2 < bf && bf < 2
    }

    fn update_height<'a, C>(cursor: &mut C)
    where
        C: BinTreeCursorMut<'a, Elem = Tree::Elem>,
    {
        let lhs = cursor.left().map_or(-1, |node| node.value.height);
        let rhs = cursor.right().map_or(-1, |node| node.value.height);
        cursor.as_mut().unwrap().value.height = lhs.max(rhs) + 1;
    }

    fn move_to_taller_child<'a, C>(cursor: &mut C) -> bool
    where
        C: BinTreeCursor<'a, Elem = Tree::Elem>,
    {
        let lhs = cursor.left().map_or(-1, |node| node.value.height);
        let rhs = cursor.right().map_or(-1, |node| node.value.height);
        if lhs > rhs {
            cursor.move_left();
            true
        } else {
            cursor.move_right();
            false
        }
    }

    /// 3 + 4重构.
    /// T T: 1 v 2 p 3 g 4
    /// T F: 1 p 2 v 3 g 4
    /// F F: 1 g 2 p 3 v 4
    /// F T: 1 g 2 v 3 p 4
    fn rebalance_at<'a, C>(cursor: &mut C, p_flag: bool, v_flag: bool)
    where
        C: MoveParentCursor<'a, Elem = Tree::Elem>
            + BinTreeCursorMut<'a, Elem = Tree::Elem, SubTree = Tree>,
        Tree: MoveParentBinTreeMut,
    {
        if p_flag && v_flag {
            // T T: 1 v 2 p 3 g 4
            let t1 = cursor.take_left();
            let t2 = cursor.take_right();
            cursor.move_parent();
            let mut v = cursor.take_left().unwrap();
            let t3 = cursor.take_right();
            cursor.move_parent();
            let mut p = cursor.take_left().unwrap();
            let t4 = cursor.take_right();
            let mut g = cursor.take();
            if let Some(t) = t1 {
                MoveParentBinTreeMut::cursor_mut(&mut v).append_left(t)
            };
            if let Some(t) = t2 {
                MoveParentBinTreeMut::cursor_mut(&mut v).append_right(t)
            };
            Self::update_height(&mut MoveParentBinTreeMut::cursor_mut(&mut v));
            if let Some(t) = t3 {
                MoveParentBinTreeMut::cursor_mut(&mut g).append_left(t)
            };
            if let Some(t) = t4 {
                MoveParentBinTreeMut::cursor_mut(&mut g).append_right(t)
            };
            Self::update_height(&mut MoveParentBinTreeMut::cursor_mut(&mut g));
            MoveParentBinTreeMut::cursor_mut(&mut p).append_left(v);
            MoveParentBinTreeMut::cursor_mut(&mut p).append_right(g);
            Self::update_height(&mut MoveParentBinTreeMut::cursor_mut(&mut p));
            cursor.append(p);
        } else if p_flag && !v_flag {
            // T F: 1 p 2 v 3 g 4
            let t2 = cursor.take_left();
            let t3 = cursor.take_right();
            cursor.move_parent();
            let mut v = cursor.take_right().unwrap();
            let t1 = cursor.take_left();
            cursor.move_parent();
            let mut p = cursor.take_left().unwrap();
            let t4 = cursor.take_right();
            let mut g = cursor.take();
            if let Some(t) = t1 {
                MoveParentBinTreeMut::cursor_mut(&mut p).append_left(t)
            };
            if let Some(t) = t2 {
                MoveParentBinTreeMut::cursor_mut(&mut p).append_right(t)
            };
            Self::update_height(&mut MoveParentBinTreeMut::cursor_mut(&mut p));
            if let Some(t) = t3 {
                MoveParentBinTreeMut::cursor_mut(&mut g).append_left(t)
            };
            if let Some(t) = t4 {
                MoveParentBinTreeMut::cursor_mut(&mut g).append_right(t)
            };
            Self::update_height(&mut MoveParentBinTreeMut::cursor_mut(&mut g));
            MoveParentBinTreeMut::cursor_mut(&mut v).append_left(p);
            MoveParentBinTreeMut::cursor_mut(&mut v).append_right(g);
            Self::update_height(&mut MoveParentBinTreeMut::cursor_mut(&mut v));
            cursor.append(v);
        } else if !p_flag && !v_flag {
            // F F: 1 g 2 p 3 v 4
            let t3 = cursor.take_left();
            let t4 = cursor.take_right();
            cursor.move_parent();
            let mut v = cursor.take_right().unwrap();
            let t2 = cursor.take_left();
            cursor.move_parent();
            let mut p = cursor.take_right().unwrap();
            let t1 = cursor.take_left();
            let mut g = cursor.take();
            if let Some(t) = t1 {
                MoveParentBinTreeMut::cursor_mut(&mut g).append_left(t)
            };
            if let Some(t) = t2 {
                MoveParentBinTreeMut::cursor_mut(&mut g).append_right(t)
            };
            Self::update_height(&mut MoveParentBinTreeMut::cursor_mut(&mut g));
            if let Some(t) = t3 {
                MoveParentBinTreeMut::cursor_mut(&mut v).append_left(t)
            };
            if let Some(t) = t4 {
                MoveParentBinTreeMut::cursor_mut(&mut v).append_right(t)
            };
            Self::update_height(&mut MoveParentBinTreeMut::cursor_mut(&mut v));
            MoveParentBinTreeMut::cursor_mut(&mut p).append_left(g);
            MoveParentBinTreeMut::cursor_mut(&mut p).append_right(v);
            Self::update_height(&mut MoveParentBinTreeMut::cursor_mut(&mut p));
            cursor.append(p);
        } else {
            // F T: 1 g 2 v 3 p 4
            let t2 = cursor.take_left();
            let t3 = cursor.take_right();
            cursor.move_parent();
            let mut v = cursor.take_left().unwrap();
            let t4 = cursor.take_right();
            cursor.move_parent();
            let mut p = cursor.take_right().unwrap();
            let t1 = cursor.take_left();
            let mut g = cursor.take();
            if let Some(t) = t1 {
                MoveParentBinTreeMut::cursor_mut(&mut g).append_left(t)
            };
            if let Some(t) = t2 {
                MoveParentBinTreeMut::cursor_mut(&mut g).append_right(t)
            };
            Self::update_height(&mut MoveParentBinTreeMut::cursor_mut(&mut g));
            if let Some(t) = t3 {
                MoveParentBinTreeMut::cursor_mut(&mut p).append_left(t)
            };
            if let Some(t) = t4 {
                MoveParentBinTreeMut::cursor_mut(&mut p).append_right(t)
            };
            Self::update_height(&mut MoveParentBinTreeMut::cursor_mut(&mut p));
            MoveParentBinTreeMut::cursor_mut(&mut v).append_left(g);
            MoveParentBinTreeMut::cursor_mut(&mut v).append_right(p);
            Self::update_height(&mut MoveParentBinTreeMut::cursor_mut(&mut v));
            cursor.append(v);
        }
    }
}

impl<Tree, K: Ord, V> Map<K, V> for AVLTreeMap<Tree, K, V>
where
    Tree: Default
        + MoveParentBinTreeMut<Elem = Entry<K, AVLNode<V>>>
        + BinTreeMut<Elem = Entry<K, AVLNode<V>>>,
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

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut parent = MoveParentBinTreeMut::cursor_mut(&mut self.bst.tree);
        let mut value = AVLNode {
            elem: value,
            height: 0,
        };
        match TreeMap::<Tree, _, _>::move_to_target(&mut parent, &key) {
            Some(Ordering::Equal) => {
                mem::swap(&mut parent.as_mut().unwrap().value, &mut value);
                parent.as_mut().unwrap().value.height = value.height;
                // 没有发生实质的结点插入，因此无需重平衡.
                return Some(value.elem);
            }
            Some(Ordering::Less) => {
                parent.insert_as_left(Entry { key, value });
                self.bst.len += 1;
            }
            Some(Ordering::Greater) => {
                parent.insert_as_right(Entry { key, value });
                self.bst.len += 1;
            }
            None => {
                parent.insert_as_root(Entry { key, value });
                self.bst.len += 1;
                // 根是树中唯一结点，因此无需平衡.
                return None;
            }
        }
        Self::update_height(&mut parent);
        // 使树重新平衡.
        while parent.parent().is_some() {
            parent.move_parent();
            if Self::is_avl_balanced(&parent) {
                Self::update_height(&mut parent);
            } else {
                let p_flag = Self::move_to_taller_child(&mut parent);
                let v_flag = Self::move_to_taller_child(&mut parent);
                Self::rebalance_at(&mut parent, p_flag, v_flag);
                break;
            }
        }

        None
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let mut cursor = MoveParentBinTreeMut::cursor_mut(&mut self.bst.tree);
        if let Some(Ordering::Equal) = TreeMap::<Tree, _, _>::move_to_target(&mut cursor, key) {
            self.bst.len -= 1;
            let elem = TreeMap::<Tree, _, _>::delete_at(&mut cursor).elem;
            // 使树重新平衡.
            while cursor.parent().is_some() {
                cursor.move_parent();
                if !Self::is_avl_balanced(&cursor) {
                    let p_flag = Self::move_to_taller_child(&mut cursor);
                    let v_flag = Self::move_to_taller_child(&mut cursor);
                    Self::rebalance_at(&mut cursor, p_flag, v_flag);
                }
                Self::update_height(&mut cursor);
            }
            Some(elem)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ch4::vec_binary_tree::VecBinaryTree;
    use crate::ch4::doubly_linked_binary_tree::DoublyLinkedBinaryTree;
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
        data.insert("Hhaha", 6);
        data.insert("Xfwawd", 7);
        data.insert("Gooo", 8);
        data.insert("jiojoij", 9);
        let mut map = AVLTreeMap::<VecBinaryTree<_>, _, _>::default();
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
        println!("{:?}", map.bst.tree.inner);
        for k in data.keys() {
            println!("{}", k);
            assert_eq!(map.remove(k), data.get(k).copied());
            println!("{:?}", map.bst.tree.inner);
        }
    }

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
        let mut map = AVLTreeMap::<DoublyLinkedBinaryTree<_>, _, _>::default();
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
            let mut map = AVLTreeMap::<DoublyLinkedBinaryTree<_>, _, _>::default();
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
