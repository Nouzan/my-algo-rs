use super::{Entry, Map};
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::mem;
use std::ptr::NonNull;

type NodePosi<K, V> = NonNull<Node<K, V>>;
type Link<K, V> = Option<NodePosi<K, V>>;

struct Elems<K: Ord, V> {
    entrys: VecDeque<Entry<K, V>>,
    childrens: VecDeque<NodePosi<K, V>>,
}

impl<K: Ord, V> Default for Elems<K, V> {
    fn default() -> Self {
        Self {
            entrys: VecDeque::new(),
            childrens: VecDeque::new(),
        }
    }
}

struct Node<K: Ord, V> {
    parent: Link<K, V>,
    elems: Elems<K, V>,
}

impl<K: Ord, V> Node<K, V> {
    fn new() -> Box<Self> {
        Box::new(Self {
            parent: None,
            elems: Elems::default(),
        })
    }

    fn leak<'a>(boxed: Box<Self>) -> NodePosi<K, V>
    where
        K: 'a,
        V: 'a,
    {
        NonNull::new(Box::leak::<'a>(boxed)).unwrap()
    }

    fn search(&self, key: &K) -> Option<usize> {
        let mut founded = None;
        for (idx, entry) in self.elems.entrys.iter().enumerate() {
            if entry.key <= *key {
                founded = Some(idx);
            } else {
                break;
            }
        }
        founded
    }

    fn split(mut boxed: Box<Self>) -> (Box<Self>, Entry<K, V>, Box<Self>) {
        let len = boxed.elems.entrys.len();
        if len < 3 {
            panic!("不能对词条数小于`3`的B树结点进行分裂.");
        } else {
            let mid = len / 2;
            let is_leaf = boxed.elems.childrens.is_empty();
            let mut left = Self::new();
            for _ in 0..mid {
                let entry = boxed.elems.entrys.pop_front().unwrap();
                left.elems.entrys.push_back(entry);
                if !is_leaf {
                    let mut posi = boxed.elems.childrens.pop_front().unwrap();
                    unsafe {
                        posi.as_mut().parent = Some(NonNull::new(left.as_mut()).unwrap());
                    }
                    left.elems.childrens.push_back(posi);
                }
            }
            let mid_entry = boxed.elems.entrys.pop_front().unwrap();
            if !is_leaf {
                let mut posi = boxed.elems.childrens.pop_front().unwrap();
                unsafe {
                    posi.as_mut().parent = Some(NonNull::new(left.as_mut()).unwrap());
                }
                left.elems.childrens.push_back(posi);
            }
            (left, mid_entry, boxed)
        }
    }

    fn merge(mut left: Box<Self>, mid: Entry<K, V>, mut right: Self) -> Box<Self> {
        left.elems.entrys.push_back(mid);
        left.elems.entrys.append(&mut right.elems.entrys);
        for mut posi in right.elems.childrens {
            unsafe {
                posi.as_mut().parent = Some(NonNull::new(left.as_mut()).unwrap());
            }
            left.elems.childrens.push_back(posi);
        }
        left
    }
}

pub struct BTreeMap<K: Ord, V, const M: usize> {
    root: NodePosi<K, V>,
    len: usize,
    marker: PhantomData<Box<Node<K, V>>>,
}

struct UnsafeCursor<K: Ord, V> {
    hot: NodePosi<K, V>,
    current: usize,
}

impl<K: Ord, V> UnsafeCursor<K, V> {
    unsafe fn current_link(&self) -> Link<K, V> {
        self.hot.as_ref().elems.childrens.get(self.current).copied()
    }

    unsafe fn get(&self, idx: usize) -> Option<&Entry<K, V>> {
        self.current_link()
            .and_then(|current| (*current.as_ptr()).elems.entrys.get(idx))
    }

    unsafe fn get_mut(&self, idx: usize) -> Option<&mut Entry<K, V>> {
        self.current_link()
            .and_then(|current| (*current.as_ptr()).elems.entrys.get_mut(idx))
    }

    unsafe fn into_ref<'a>(self, idx: usize) -> Option<&'a Entry<K, V>> {
        self.current_link()
            .and_then(|current| (*current.as_ptr()).elems.entrys.get(idx))
    }

    unsafe fn into_mut<'a>(self, idx: usize) -> Option<&'a mut Entry<K, V>> {
        self.current_link()
            .and_then(|current| (*current.as_ptr()).elems.entrys.get_mut(idx))
    }

    unsafe fn move_to(&mut self, next: usize) {
        if let Some(current) = self.current_link() {
            self.hot = current;
            self.current = next;
        }
    }

    unsafe fn search(&self, key: &K) -> Option<usize> {
        if let Some(current) = self.current_link() {
            current.as_ref().search(key)
        } else {
            None
        }
    }

    unsafe fn is_leaf(&self) -> bool {
        if let Some(current) = self.current_link() {
            current.as_ref().elems.childrens.is_empty()
        } else {
            false
        }
    }
}

pub struct Iter<'a, K: Ord, V, const M: usize> {
    current: Link<K, V>,
    rank: usize,
    stack: Vec<(NodePosi<K, V>, usize)>,
    marker: PhantomData<&'a BTreeMap<K, V, M>>,
}

impl<'a, K: Ord, V, const M: usize> Iter<'a, K, V, M> {
    fn push_left_most_chain(&mut self) {
        unsafe {
            while let Some(current) = self.current {
                if self.rank < current.as_ref().elems.entrys.len() {
                    self.stack.push((current, self.rank));
                }
                self.current = current.as_ref().elems.childrens.get(self.rank).copied();
                self.rank = 0;
            }
            if let Some((next, rank)) = self.stack.pop() {
                self.current = Some(next);
                self.rank = rank;
            }
        }
    }

    pub fn new(map: &'a BTreeMap<K, V, M>) -> Self {
        let mut iter = Self {
            current: unsafe { map.unsafe_cursor().current_link() },
            rank: 0,
            stack: Vec::default(),
            marker: PhantomData::default(),
        };
        iter.push_left_most_chain();
        iter
    }
}

impl<'a, K: Ord, V, const M: usize> Iterator for Iter<'a, K, V, M> {
    type Item = &'a Entry<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if let Some(current) = self.current {
                let target = current;
                let rank = self.rank;
                self.rank += 1;
                self.push_left_most_chain();
                let entry = (*target.as_ptr()).elems.entrys.get(rank).unwrap();
                Some(entry)
            } else {
                None
            }
        }
    }
}

impl<K: Ord, V, const M: usize> BTreeMap<K, V, M> {
    pub fn new() -> Self {
        let node = Box::new(Node {
            parent: None,
            elems: Elems::default(),
        });
        Self {
            root: Node::leak(node),
            len: 0,
            marker: PhantomData::default(),
        }
    }

    fn unsafe_cursor(&self) -> UnsafeCursor<K, V> {
        UnsafeCursor {
            hot: self.root,
            current: 0,
        }
    }

    unsafe fn move_to_target(cursor: &mut UnsafeCursor<K, V>, key: &K) -> Option<usize> {
        while cursor.current_link().is_some() {
            if let Some(idx) = cursor.search(key) {
                let entry = cursor.get(idx).unwrap();
                if entry.key == *key {
                    return Some(idx);
                } else {
                    cursor.move_to(idx + 1);
                }
            } else {
                cursor.move_to(0);
            }
        }
        None
    }

    unsafe fn solve_overflow(&mut self, cursor: UnsafeCursor<K, V>) {
        // 刚刚插入的词条在`cursor.hot`所指结点的`cursor.current`处.
        // `cursor.hot`是叶子.
        let mut hot = cursor.hot;
        while hot.as_ref().elems.entrys.len() + 1 > M {
            let node = Box::from_raw(hot.as_ptr());
            hot = if node.parent != Some(self.root) {
                node.parent.unwrap()
            } else {
                let mut node = Node::new();
                node.parent = Some(self.root);
                let parent = Node::leak(node);
                *self.root.as_mut().elems.childrens.get_mut(0).unwrap() = parent;
                parent
            };
            let (mut left, mid, mut right) = Node::split(node);
            let rank = if let Some(idx) = hot.as_ref().search(&mid.key) {
                idx + 1
            } else {
                0
            };
            left.parent = Some(hot);
            right.parent = Some(hot);
            hot.as_mut().elems.entrys.insert(rank, mid);
            hot.as_mut().elems.childrens.insert(rank, Node::leak(left));
            if let Some(child) = hot.as_mut().elems.childrens.get_mut(rank + 1) {
                *child = Node::leak(right);
            } else {
                hot.as_mut().elems.childrens.push_back(Node::leak(right));
            }
        }
    }

    unsafe fn solve_underflow(&mut self, mut cursor: UnsafeCursor<K, V>) {
        while (cursor.hot == self.root
            && cursor
                .current_link()
                .unwrap()
                .as_ref()
                .elems
                .entrys
                .is_empty())
            || (cursor.hot != self.root
                && cursor.current_link().unwrap().as_ref().elems.entrys.len() + 1 < (M + 1) / 2)
        {
            // 判断是否为根
            if cursor.hot == self.root {
                let root = self.root.as_mut().elems.childrens.pop_front().unwrap();
                let mut root = Box::from_raw(root.as_ptr());
                if let Some(mut posi) = root.elems.childrens.pop_front() {
                    self.root.as_mut().elems.childrens.push_back(posi);
                    posi.as_mut().parent = Some(self.root);
                }
                break;
            } else {
                let left = if cursor.current > 0 {
                    // 找左兄弟
                    let mut left = cursor
                        .hot
                        .as_ref()
                        .elems
                        .childrens
                        .get(cursor.current - 1)
                        .copied()
                        .unwrap();
                    if left.as_ref().elems.entrys.len() + 1 >= (M + 1) / 2 + 1 {
                        let left_entry = left.as_mut().elems.entrys.back_mut().unwrap();
                        let parent_entry = cursor
                            .hot
                            .as_mut()
                            .elems
                            .entrys
                            .get_mut(cursor.current - 1)
                            .unwrap();
                        mem::swap(left_entry, parent_entry);
                        let entry = left.as_mut().elems.entrys.pop_back().unwrap();
                        cursor
                            .current_link()
                            .unwrap()
                            .as_mut()
                            .elems
                            .entrys
                            .push_front(entry);
                        if let Some(mut posi) = left.as_mut().elems.childrens.pop_back() {
                            posi.as_mut().parent = cursor.current_link();
                            cursor
                                .current_link()
                                .unwrap()
                                .as_mut()
                                .elems
                                .childrens
                                .push_front(posi);
                        }
                        break;
                    } else {
                        Some(left)
                    }
                } else {
                    None
                };
                let right = if let Some(mut right) = cursor
                    .hot
                    .as_ref()
                    .elems
                    .childrens
                    .get(cursor.current + 1)
                    .copied()
                {
                    // 找右兄弟
                    if right.as_ref().elems.entrys.len() + 1 >= (M + 1) / 2 + 1 {
                        let right_entry = right.as_mut().elems.entrys.front_mut().unwrap();
                        let parent_entry = cursor
                            .hot
                            .as_mut()
                            .elems
                            .entrys
                            .get_mut(cursor.current)
                            .unwrap();
                        mem::swap(right_entry, parent_entry);
                        let entry = right.as_mut().elems.entrys.pop_front().unwrap();
                        cursor
                            .current_link()
                            .unwrap()
                            .as_mut()
                            .elems
                            .entrys
                            .push_back(entry);
                        if let Some(mut posi) = right.as_mut().elems.childrens.pop_front() {
                            posi.as_mut().parent = cursor.current_link();
                            cursor
                                .current_link()
                                .unwrap()
                                .as_mut()
                                .elems
                                .childrens
                                .push_back(posi);
                        }
                        break;
                    } else {
                        Some(right)
                    }
                } else {
                    None
                };
                if let Some(left) = left {
                    let entry = cursor
                        .hot
                        .as_mut()
                        .elems
                        .entrys
                        .remove(cursor.current - 1)
                        .unwrap();

                    let parent = cursor.hot.as_ref().parent.unwrap();
                    let key = &entry.key;
                    let rank = if let Some(idx) = parent.as_ref().search(key) {
                        idx + 1
                    } else {
                        0
                    };
                    let current = cursor
                        .hot
                        .as_mut()
                        .elems
                        .childrens
                        .remove(cursor.current)
                        .unwrap();
                    let mut merged = Node::merge(
                        Box::from_raw(left.as_ptr()),
                        entry,
                        *Box::from_raw(current.as_ptr()),
                    );
                    merged.parent = Some(cursor.hot);
                    *cursor
                        .hot
                        .as_mut()
                        .elems
                        .childrens
                        .get_mut(cursor.current - 1)
                        .unwrap() = Node::leak(merged);
                    cursor.hot = parent;
                    cursor.current = rank;
                } else if let Some(right) = right {
                    let entry = cursor
                        .hot
                        .as_mut()
                        .elems
                        .entrys
                        .remove(cursor.current)
                        .unwrap();
                    let parent = cursor.hot.as_ref().parent.unwrap();
                    let key = &entry.key;
                    let rank = if let Some(idx) = parent.as_ref().search(key) {
                        idx + 1
                    } else {
                        0
                    };
                    let current = cursor.current_link().unwrap();
                    cursor
                        .hot
                        .as_mut()
                        .elems
                        .childrens
                        .remove(cursor.current + 1)
                        .unwrap();
                    let mut merged = Node::merge(
                        Box::from_raw(current.as_ptr()),
                        entry,
                        *Box::from_raw(right.as_ptr()),
                    );
                    merged.parent = Some(cursor.hot);
                    *cursor
                        .hot
                        .as_mut()
                        .elems
                        .childrens
                        .get_mut(cursor.current)
                        .unwrap() = Node::leak(merged);
                    cursor.hot = parent;
                    cursor.current = rank;
                } else {
                    panic!("impossible!")
                }
            }
        }
    }
}

impl<K: Ord, V, const M: usize> Drop for BTreeMap<K, V, M> {
    fn drop(&mut self) {
        unsafe fn drop_inner<K: Ord, V>(root: Node<K, V>) {
            for child in root.elems.childrens {
                let node = Box::from_raw(child.as_ptr());
                drop_inner(*node);
            }
        }
        unsafe {
            let mut node = Box::from_raw(self.root.as_ptr());
            // 根结点是`node`的第一个孩子.
            if let Some(root) = node.elems.childrens.pop_front() {
                let root = Box::from_raw(root.as_ptr());
                drop_inner(*root);
            }
        }
    }
}

impl<K: Ord, V, const M: usize> Default for BTreeMap<K, V, M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V, const M: usize> Map<K, V> for BTreeMap<K, V, M> {
    fn len(&self) -> usize {
        self.len
    }

    fn get(&self, key: &K) -> Option<&V> {
        unsafe {
            let mut cursor = self.unsafe_cursor();
            Self::move_to_target(&mut cursor, key)
                .and_then(|idx| cursor.into_ref(idx).map(|entry| &entry.value))
        }
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        unsafe {
            let mut cursor = self.unsafe_cursor();
            Self::move_to_target(&mut cursor, key)
                .and_then(|idx| cursor.into_mut(idx).map(|entry| &mut entry.value))
        }
    }

    fn get_mut_or_insert(&mut self, key: K, default: V) -> &mut V {
        unsafe {
            let mut cursor = self.unsafe_cursor();
            if cursor.current_link().is_some() {
                if let Some(idx) = Self::move_to_target(&mut cursor, &key) {
                    cursor.into_mut(idx).map(|entry| &mut entry.value).unwrap()
                } else {
                    cursor.hot.as_mut().elems.entrys.insert(
                        cursor.current,
                        Entry {
                            key,
                            value: default,
                        },
                    );
                    // Safety: 必须非常小心地验证`value_ptr`不会在`solve_overflow`过程中发生`move`.
                    let value_ptr = &mut (*cursor.hot.as_ptr())
                        .elems
                        .entrys
                        .get_mut(cursor.current)
                        .unwrap()
                        .value;
                    self.solve_overflow(cursor);
                    self.len += 1;
                    value_ptr
                }
            } else {
                let mut elems = Elems::default();
                elems.entrys.push_front(Entry {
                    key,
                    value: default,
                });
                let node = Box::new(Node {
                    parent: Some(self.root),
                    elems,
                });
                let posi = Node::leak(node);
                self.root.as_mut().elems.childrens.push_front(posi);
                self.len += 1;
                &mut (*posi.as_ptr()).elems.entrys.get_mut(0).unwrap().value
            }
        }
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        unsafe {
            let mut cursor = self.unsafe_cursor();
            if cursor.current_link().is_some() {
                if let Some(idx) = Self::move_to_target(&mut cursor, &key) {
                    let mut entry = Entry { key, value };
                    mem::swap(&mut entry, cursor.get_mut(idx).unwrap());
                    Some(entry.value)
                } else {
                    cursor
                        .hot
                        .as_mut()
                        .elems
                        .entrys
                        .insert(cursor.current, Entry { key, value });
                    self.solve_overflow(cursor);
                    self.len += 1;
                    None
                }
            } else {
                let mut elems = Elems::default();
                elems.entrys.push_front(Entry { key, value });
                let node = Box::new(Node {
                    parent: Some(self.root),
                    elems,
                });
                self.root
                    .as_mut()
                    .elems
                    .childrens
                    .push_front(Node::leak(node));
                self.len += 1;
                None
            }
        }
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        unsafe {
            let mut cursor = self.unsafe_cursor();
            if let Some(mut idx) = Self::move_to_target(&mut cursor, key) {
                if !cursor.is_leaf() {
                    let current = cursor.current_link().unwrap();
                    cursor.move_to(idx + 1);
                    while !cursor.is_leaf() {
                        cursor.move_to(0);
                    }
                    let entry = cursor.get_mut(0).unwrap();
                    mem::swap(
                        (*current.as_ptr()).elems.entrys.get_mut(idx).unwrap(),
                        entry,
                    );
                    idx = 0;
                }
                let entry = cursor
                    .current_link()
                    .unwrap()
                    .as_mut()
                    .elems
                    .entrys
                    .remove(idx)
                    .unwrap();
                self.solve_underflow(cursor);
                self.len -= 1;
                Some(entry.value)
            } else {
                None
            }
        }
    }

    fn iter<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = (&K, &V)>> {
        Box::new(Iter::new(self).map(|entry| (&entry.key, &entry.value)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;
    use std::collections::HashMap;

    #[test]
    fn test_map_broken() {
        let mut data = HashMap::new();
        let random = "";
        data.insert("a", 1);
        data.insert("b", 2);
        data.insert("c", 3);
        data.insert("d", 4);
        data.insert("e", 5);
        data.insert("f", 6);
        data.insert("g", 7);
        data.insert("h", 8);
        data.insert("i", 9);
        let mut map = BTreeMap::<_, _, 5>::default();
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
        for (k, v) in map.iter() {
            println!("{} {}", k, v);
        }
        // remove
        for (idx, k) in data.keys().enumerate() {
            println!("{}", k);
            assert_eq!(map.remove(k), data.get(k).copied());
            assert_eq!(map.len(), data.len() - idx - 1);
        }
        assert!(map.is_empty());
    }

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
        let mut map = BTreeMap::<_, _, 4>::default();
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
    }

    proptest! {
        #[test]
        fn test_btree_map(mut data: HashMap<i64, i64>, random: i64) {
            let mut map = BTreeMap::<_, _, 3>::default();
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
