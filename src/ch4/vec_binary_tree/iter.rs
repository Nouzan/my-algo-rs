use super::VecBinaryTree;

const fn round_down_to_power_of_two(n: usize) -> usize {
    if let Some(n) = usize::MAX.checked_shr(n.leading_zeros() + 1) {
        n + 1
    } else {
        1
    }
}

/// 层序遍历下标公式.
/// `base`为根下标，`index`为相对于根的下标(根为`0`).
pub const fn in_order_index(base: usize, index: usize) -> usize {
    let pow = round_down_to_power_of_two(index + 1);
    index + base * pow
}

pub(super) struct InOrderIndexIter {
    base: usize,
    index: usize,
    limit: usize,
}

impl InOrderIndexIter {
    pub(super) fn new(current: usize, limit: usize) -> Self {
        Self {
            base: current,
            index: 0,
            limit,
        }
    }
}

impl Iterator for InOrderIndexIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = in_order_index(self.base, self.index);
        if idx < self.limit {
            self.index += 1;
            Some(idx)
        } else {
            None
        }
    }
}

pub struct InOrderIter<'a, T> {
    iter: InOrderIndexIter,
    tree: &'a VecBinaryTree<T>,
}

impl<'a, T> InOrderIter<'a, T> {
    pub fn new(base: usize, tree: &'a VecBinaryTree<T>) -> Self {
        Self {
            iter: InOrderIndexIter::new(base, tree.inner.len()),
            tree,
        }
    }
}

impl<'a, T> Iterator for InOrderIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(idx) = self.iter.next() {
            if let Some(elem) = self.tree.get(idx) {
                return Some(elem);
            }
        }

        None
    }
}
