use super::Map;
use std::cmp::Ordering;

type Link<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug)]
pub(super) struct Node<K, V> {
    pub left: Link<K, V>,
    pub right: Link<K, V>,
    pub key: K,
    pub value: V,
    pub size: usize,
}

impl<K, V> Node<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self {
            left: None,
            right: None,
            key,
            value,
            size: 1,
        }
    }

    pub fn size(link: &Link<K, V>) -> usize {
        match link {
            None => 0,
            Some(node) => node.size,
        }
    }
}

impl<K: Ord, V> Node<K, V> {
    fn get<'a>(link: &'a Link<K, V>, key: &K) -> Option<&'a V> {
        if let Some(node) = link {
            match key.cmp(&node.key) {
                Ordering::Less => Self::get(&node.left, key),
                Ordering::Greater => Self::get(&node.right, key),
                Ordering::Equal => Some(&node.value),
            }
        } else {
            None
        }
    }

    fn get_mut<'a>(link: &'a mut Link<K, V>, key: &K) -> Option<&'a mut V> {
        if let Some(node) = link {
            match key.cmp(&node.key) {
                Ordering::Less => Self::get_mut(&mut node.left, key),
                Ordering::Greater => Self::get_mut(&mut node.right, key),
                Ordering::Equal => Some(&mut node.value),
            }
        } else {
            None
        }
    }

    fn search_mut<'a>(&'a mut self, key: &K) -> (&'a mut Self, Ordering) {
        match key.cmp(&self.key) {
            Ordering::Less => {
                if self.left.is_some() {
                    let (value, ordering) = Self::search_mut(self.left.as_mut().unwrap(), key);
                    if !matches!(ordering, Ordering::Equal) {
                        self.size += 1
                    }
                    (value, ordering)
                } else {
                    self.size += 1;
                    (self, Ordering::Less)
                }
            }
            Ordering::Greater => {
                if self.right.is_some() {
                    let (value, ordering) = Self::search_mut(self.right.as_mut().unwrap(), key);
                    if !matches!(ordering, Ordering::Equal) {
                        self.size += 1
                    }
                    (value, ordering)
                } else {
                    self.size += 1;
                    (self, Ordering::Greater)
                }
            }
            Ordering::Equal => (self, Ordering::Equal),
        }
    }

    fn insert(link: Link<K, V>, key: K, value: V) -> (Box<Self>, Option<V>) {
        if let Some(mut node) = link {
            let value = match key.cmp(&node.key) {
                Ordering::Less => {
                    let (child, value) = Self::insert(node.left, key, value);
                    node.left = Some(child);
                    value
                }
                Ordering::Greater => {
                    let (child, value) = Self::insert(node.right, key, value);
                    node.right = Some(child);
                    value
                }
                Ordering::Equal => {
                    let old = node.value;
                    node.value = value;
                    Some(old)
                }
            };
            node.size = Self::size(&node.left) + Self::size(&node.right) + 1;
            (node, value)
        } else {
            (Box::new(Self::new(key, value)), None)
        }
    }

    fn delete_min(mut self: Box<Self>) -> (Link<K, V>, Box<Self>) {
        if let Some(node) = self.left {
            let (left, deleted) = node.delete_min();
            self.left = left;
            self.size = Self::size(&self.left) + Self::size(&self.right) + 1;
            (Some(self), deleted)
        } else {
            (self.right.take(), self)
        }
    }

    fn delete(mut self: Box<Self>, key: &K) -> (Link<K, V>, Link<K, V>) {
        match key.cmp(&self.key) {
            Ordering::Less => {
                if let Some(left) = self.left {
                    let (left, deleted) = left.delete(key);
                    self.left = left;
                    self.size = Self::size(&self.left) + Self::size(&self.right) + 1;
                    (Some(self), deleted)
                } else {
                    (Some(self), None)
                }
            }
            Ordering::Greater => {
                if let Some(right) = self.right {
                    let (right, deleted) = right.delete(key);
                    self.right = right;
                    self.size = Self::size(&self.left) + Self::size(&self.right) + 1;
                    (Some(self), deleted)
                } else {
                    (Some(self), None)
                }
            }
            Ordering::Equal => {
                if self.left.is_some() && self.right.is_some() {
                    let (right, mut deleted) = self.right.take().unwrap().delete_min();
                    self.right = right;
                    let key = deleted.key;
                    let value = deleted.value;
                    deleted.key = self.key;
                    deleted.value = self.value;
                    self.key = key;
                    self.value = value;
                    self.size = Self::size(&self.left) + Self::size(&self.right) + 1;
                    (Some(self), Some(deleted))
                } else if let Some(left) = self.left.take() {
                    (Some(left), Some(self))
                } else if let Some(right) = self.right.take() {
                    (Some(right), Some(self))
                } else {
                    (None, Some(self))
                }
            }
        }
    }
}

pub struct TreeMap<K: Ord, V> {
    pub(super) root: Link<K, V>,
}

impl<K: Ord, V> Default for TreeMap<K, V> {
    fn default() -> Self {
        Self { root: None }
    }
}

impl<K: Ord, V> Map<K, V> for TreeMap<K, V> {
    fn get(&self, key: &K) -> Option<&V> {
        Node::get(&self.root, key)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        Node::get_mut(&mut self.root, key)
    }

    fn get_mut_or_insert(&mut self, key: K, default: V) -> &mut V {
        if self.root.is_some() {
            let (node, ordering) = Node::search_mut(self.root.as_mut().unwrap(), &key);
            match ordering {
                Ordering::Equal => &mut node.value,
                Ordering::Less => {
                    let child = Box::new(Node::new(key, default));
                    node.left = Some(child);
                    &mut node.left.as_mut().unwrap().value
                }
                Ordering::Greater => {
                    let child = Box::new(Node::new(key, default));
                    node.right = Some(child);
                    &mut node.left.as_mut().unwrap().value
                }
            }
        } else {
            let child = Box::new(Node::new(key, default));
            self.root = Some(child);
            &mut self.root.as_mut().unwrap().value
        }
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let (node, value) = Node::insert(self.root.take(), key, value);
        self.root = Some(node);
        value
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(root) = self.root.take() {
            let (root, deleted) = root.delete(key);
            self.root = root;
            deleted.map(|node| node.value)
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        Node::size(&self.root)
    }

    fn iter<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = (&K, &V)>> {
        Box::new(Iter::new(self))
    }
}

pub struct Iter<'a, K, V> {
    current: Option<&'a Node<K, V>>,
    stack: Vec<&'a Node<K, V>>,
}

impl<'a, K: Ord, V> Iter<'a, K, V> {
    pub fn new(tree: &'a TreeMap<K, V>) -> Self {
        let mut iter = Self {
            current: None,
            stack: Vec::new(),
        };
        iter.push_left_chain(tree.root.as_deref());
        iter.current = iter.stack.pop();
        iter
    }

    fn push_left_chain(&mut self, mut current: Option<&'a Node<K, V>>) {
        while let Some(node) = current {
            self.stack.push(node);
            current = node.left.as_deref();
        }
    }
}

impl<'a, K: Ord, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.current.take() {
            let key = &node.key;
            let value = &node.value;
            self.push_left_chain(node.right.as_deref());
            self.current = self.stack.pop();
            Some((key, value))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
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
        let mut map = TreeMap::<_, _>::default();
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
        fn test_map_basic_proptest(mut data: HashMap<String, i64>, random: String) {
            let mut map = TreeMap::<_, _>::default();
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
