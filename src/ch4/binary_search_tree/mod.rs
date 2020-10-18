pub mod avl;
pub mod bst;

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

    /// 移除一个键值对，并返回它的值.
    fn remove(&mut self, key: &K) -> Option<V>;

    /// 返回字典的大小.
    fn len(&self) -> usize;

    /// 字典是否为空.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
