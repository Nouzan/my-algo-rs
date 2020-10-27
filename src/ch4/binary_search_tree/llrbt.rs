use super::{
    bst2::{self, TreeMap},
    Map,
};
use std::cmp::Ordering;

type Link<K, V> = Option<BoxedNode<K, V>>;
type BoxedNode<K, V> = Box<bst2::Node<K, Node<V>>>;

const RED: bool = true;
const BLACK: bool = false;

struct Node<T> {
    elem: T,
    color: bool,
}

fn is_red<K: Ord, V>(link: &Link<K, V>) -> bool {
    if let Some(node) = link {
        node.value.color == RED
    } else {
        false
    }
}

fn size<K: Ord, V>(link: &Link<K, V>) -> usize {
    bst2::Node::size(link)
}

fn zig<K: Ord, V>(mut node: BoxedNode<K, V>) -> BoxedNode<K, V> {
    let mut x = node.right.take().unwrap();
    node.right = x.left.take();
    x.value.color = node.value.color;
    node.value.color = RED;
    x.size = node.size;
    node.size = 1 + size(&node.left) + size(&node.right);
    x.left = Some(node);
    x
}

fn zag<K: Ord, V>(mut node: BoxedNode<K, V>) -> BoxedNode<K, V> {
    let mut x = node.left.take().unwrap();
    node.left = x.right.take();
    x.value.color = node.value.color;
    node.value.color = RED;
    x.size = node.size;
    node.size = 1 + size(&node.left) + size(&node.right);
    x.right = Some(node);
    x
}

fn flip<K: Ord, V>(node: &mut BoxedNode<K, V>) {
    node.value.color = !node.value.color;
    node.left.as_mut().unwrap().value.color = !node.left.as_mut().unwrap().value.color;
    node.right.as_mut().unwrap().value.color = !node.right.as_mut().unwrap().value.color;
}

fn new_node<K: Ord, V>(key: K, value: V, color: bool) -> BoxedNode<K, V> {
    Box::new(bst2::Node::new(key, Node { elem: value, color }))
}

fn insert<K: Ord, V>(link: Link<K, V>, key: K, value: V) -> (BoxedNode<K, V>, Option<V>) {
    if let Some(mut node) = link {
        let value = match key.cmp(&node.key) {
            Ordering::Less => {
                let (left, value) = insert(node.left.take(), key, value);
                node.left = Some(left);
                value
            }
            Ordering::Greater => {
                let (right, value) = insert(node.right.take(), key, value);
                node.right = Some(right);
                value
            }
            Ordering::Equal => {
                let old = node.value.elem;
                node.value.elem = value;
                Some(old)
            }
        };

        if is_red(&node.right) && !is_red(&node.left) {
            node = zig(node)
        }
        if is_red(&node.left) && is_red(&node.left.as_ref().unwrap().left) {
            node = zag(node)
        }
        if is_red(&node.left) && is_red(&node.right) {
            flip(&mut node)
        }

        node.size = size(&node.left) + size(&node.right) + 1;

        (node, value)
    } else {
        (new_node(key, value, RED), None)
    }
}

fn get_mut_or_insert<'a, K: Ord, V>(
    link: Link<K, V>,
    key: K,
    value: V,
) -> (*mut V, BoxedNode<K, V>) {
    if let Some(mut node) = link {
        let target = match key.cmp(&node.key) {
            Ordering::Less => {
                let (target, left) = get_mut_or_insert(node.left.take(), key, value);
                node.left = Some(left);
                target
            }
            Ordering::Greater => {
                let (target, right) = get_mut_or_insert(node.right.take(), key, value);
                node.right = Some(right);
                target
            }
            Ordering::Equal => &mut node.value.elem as *mut _,
        };

        (target, fix_up(node))
    } else {
        let mut node = new_node(key, value, RED);
        (&mut node.value.elem as *mut _, node)
    }
}

fn fix_up<K: Ord, V>(mut node: BoxedNode<K, V>) -> BoxedNode<K, V> {
    if is_red(&node.right) && !is_red(&node.left) {
        node = zig(node)
    }
    if is_red(&node.left) && is_red(&node.left.as_ref().unwrap().left) {
        node = zag(node)
    }
    if is_red(&node.left) && is_red(&node.right) {
        flip(&mut node)
    }
    node.size = size(&node.left) + size(&node.right) + 1;
    node
}

fn move_red_left<K: Ord, V>(mut node: BoxedNode<K, V>) -> BoxedNode<K, V> {
    flip(&mut node);
    if is_red(&node.right.as_ref().unwrap().left) {
        node.right = Some(zag(node.right.take().unwrap()));
        node = zig(node);
        flip(&mut node);
    }
    node
}

fn move_red_right<K: Ord, V>(mut node: BoxedNode<K, V>) -> BoxedNode<K, V> {
    flip(&mut node);
    if is_red(&node.left.as_ref().unwrap().left) {
        node = zag(node);
        flip(&mut node);
    }
    node
}

fn delete_min<K: Ord, V>(mut node: BoxedNode<K, V>) -> (Link<K, V>, BoxedNode<K, V>) {
    if node.left.is_none() {
        (None, node)
    } else {
        if !is_red(&node.left) && !is_red(&node.left.as_ref().unwrap().left) {
            node = move_red_left(node);
        }
        let (left, deleted) = delete_min(node.left.take().unwrap());
        node.left = left;

        (Some(fix_up(node)), deleted)
    }
}

fn delete<K: Ord, V>(mut node: BoxedNode<K, V>, key: &K) -> (Link<K, V>, Link<K, V>) {
    match key.cmp(&node.key) {
        Ordering::Less => {
            if node.left.is_none() {
                return (Some(node), None);
            }
            if !is_red(&node.left) && !is_red(&node.left.as_ref().unwrap().left) {
                node = move_red_left(node);
            }
            let (left, deleted) = delete(node.left.take().unwrap(), key);
            node.left = left;

            (Some(fix_up(node)), deleted)
        }
        _ => {
            if is_red(&node.left) {
                // 为了确保不会出现，右边为空，但左边不为空的情况.
                node = zag(node);
            }
            let ordering = key.cmp(&node.key);
            if matches!(ordering, Ordering::Equal) && node.right.is_none() {
                // 左边必为空.
                return (None, Some(node));
            }
            if matches!(ordering, Ordering::Greater) && node.right.is_none() {
                return (Some(fix_up(node)), None);
            }
            if !is_red(&node.right) && !is_red(&node.right.as_ref().unwrap().left) {
                node = move_red_right(node);
            }
            let ordering = key.cmp(&node.key);
            if matches!(ordering, Ordering::Equal) {
                // 右子树的根或它的左孩子之一必然是红的.(由`move_red_right`保证)
                let (right, mut deleted) = delete_min(node.right.take().unwrap());
                node.right = right;
                let key = node.key;
                let value = node.value.elem;
                node.key = deleted.key;
                node.value.elem = deleted.value.elem;
                deleted.key = key;
                deleted.value.elem = value;
                (Some(fix_up(node)), Some(deleted))
            } else {
                let (right, deleted) = delete(node.right.take().unwrap(), key);
                node.right = right;
                (Some(fix_up(node)), deleted)
            }
        }
    }
}

pub struct RBTreeMap<K: Ord, V> {
    bst: TreeMap<K, Node<V>>,
}

impl<K: Ord, V> Default for RBTreeMap<K, V> {
    fn default() -> Self {
        Self {
            bst: TreeMap::default(),
        }
    }
}

impl<K: Ord, V> Map<K, V> for RBTreeMap<K, V> {
    fn get(&self, key: &K) -> Option<&V> {
        self.bst.get(key).map(|node| &node.elem)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.bst.get_mut(key).map(|node| &mut node.elem)
    }

    fn len(&self) -> usize {
        self.bst.len()
    }

    fn iter<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = (&K, &V)>> {
        Box::new(self.bst.iter().map(|(k, v)| (k, &v.elem)))
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let (mut root, value) = insert(self.bst.root.take(), key, value);
        root.value.color = BLACK;
        self.bst.root = Some(root);
        value
    }

    fn get_mut_or_insert(&mut self, key: K, default: V) -> &mut V {
        let (target, mut root) = get_mut_or_insert(self.bst.root.take(), key, default);
        root.value.color = BLACK;
        self.bst.root = Some(root);
        unsafe { &mut *target }
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(root) = self.bst.root.take() {
            let (mut root, deleted) = delete(root, key);
            root.as_mut().map(|node| node.value.color = BLACK);
            self.bst.root = root;
            deleted.map(|node| node.value.elem)
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
        data.insert(1, 1);
        data.insert(2, 2);
        data.insert(3, 3);
        data.insert(4, 4);
        data.insert(5, 5);
        let mut map = RBTreeMap::<_, _>::default();
        for (k, v) in data.clone() {
            println!("{}", k);
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
            println!("{}", k);
            assert_eq!(map.remove(k), data.get(k).copied())
        }
    }

    proptest! {
        #[test]
        fn test_map_basic_proptest(mut data: HashMap<String, i64>, random: String) {
            let mut map = RBTreeMap::<_, _>::default();
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
