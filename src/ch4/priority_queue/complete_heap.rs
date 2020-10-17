use super::PriorityQueue;
use crate::vec::MyVec;
use std::convert::From;

pub struct CompleteMaxHeap<T> {
    vec: MyVec<T>,
}

impl<T> Default for CompleteMaxHeap<T> {
    fn default() -> Self {
        Self {
            vec: MyVec::default(),
        }
    }
}

impl<T> CompleteMaxHeap<T> {
    const fn left(n: usize) -> usize {
        2 * n + 1
    }
    const fn right(n: usize) -> usize {
        2 * n + 2
    }
    const fn parent(n: usize) -> usize {
        (n - 1) >> 1
    }
}

impl<T: PartialOrd> CompleteMaxHeap<T> {
    /// 上滤.
    /// 若父母小于孩子则交换，并继续向上比较.
    /// # Panics
    /// `n`溢出时报错.
    fn percolate_up(&mut self, mut n: usize) {
        while n != 0 {
            let parent = Self::parent(n);
            if self.vec.get(parent).unwrap() < self.vec.get(n).unwrap() {
                self.vec.swap(n, parent);
                n = parent;
            } else {
                break;
            }
        }
    }

    /// 下滤(直到`limit`).
    /// 若父母小于孩子则交换，并继续向下比较，直到越界.
    fn percolate_down_with_limit(&mut self, mut n: usize, limit: usize) {
        while n < limit {
            let mut max = n;
            let left = Self::left(n);
            if left < limit {
                if self.vec.get(max).unwrap() < self.vec.get(left).unwrap() {
                    max = left;
                }
            }
            let right = Self::right(n);
            if right < limit {
                if self.vec.get(max).unwrap() < self.vec.get(right).unwrap() {
                    max = right;
                }
            }
            if max != n {
                self.vec.swap(max, n);
                n = max;
            } else {
                break;
            }
        }
    }

    /// 下滤.
    /// 若父母小于孩子则交换，并继续向下比较，直到越界.
    fn percolate_down(&mut self, n: usize) {
        self.percolate_down_with_limit(n, self.vec.len())
    }

    /// 堆排序.
    /// 消耗一个列表，并返回一个排序好的列表.
    pub fn sort(vec: MyVec<T>) -> MyVec<T> {
        let mut heap = Self::from(vec);
        for idx in (0..(heap.vec.len())).rev() {
            heap.vec.swap(0, idx);
            heap.percolate_down_with_limit(0, idx);
        }
        heap.vec
    }

    /// 重新建堆.
    fn rebuild(&mut self) {
        if self.vec.len() > 1 {
            let last = self.vec.len() - 1;
            for idx in (0..=Self::parent(last)).rev() {
                self.percolate_down(idx);
            }
        }
    }
}

impl<T: PartialOrd> PriorityQueue<T> for CompleteMaxHeap<T> {
    /// 插入一个新的元素.
    fn insert(&mut self, elem: T) {
        self.vec.push(elem);
        self.percolate_up(self.vec.len() - 1);
    }

    /// 删除最大元素.
    /// 若堆空则返回`None`.
    fn delete_max(&mut self) -> Option<T> {
        if self.vec.is_empty() {
            None
        } else {
            let last = self.vec.len() - 1;
            self.vec.swap(0, last);
            let elem = self.vec.pop();
            self.percolate_down(0);
            elem
        }
    }

    fn len(&self) -> usize {
        self.vec.len()
    }

    fn get_max(&self) -> Option<&T> {
        self.vec.get(0)
    }

    fn merge(&mut self, mut other: Self) {
        while let Some(elem) = other.vec.pop() {
            self.vec.push(elem);
        }
        self.rebuild();
    }
}

impl<T: PartialOrd> From<MyVec<T>> for CompleteMaxHeap<T> {
    fn from(vec: MyVec<T>) -> Self {
        let mut heap = Self { vec };
        heap.rebuild();
        heap
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_basic(mut data: Vec<i64>) {
            let sorted = CompleteMaxHeap::sort(MyVec::from(data.clone()));
            data.sort();
            for (idx, &elem) in sorted.iter().enumerate() {
                prop_assert_eq!(data[idx], elem);
            }
        }

        #[test]
        fn test_priority(data1: Vec<i64>, data2: Vec<i64>) {
            let mut heap = CompleteMaxHeap::from(MyVec::from(data1.clone()));
            for &elem in data2.iter() {
                heap.insert(elem);
                let max = heap.vec.iter().max().copied();
                prop_assert_eq!(heap.delete_max(), max);
            }
        }

        #[test]
        fn test_merge(data1: Vec<i64>, data2: Vec<i64>) {
            let mut heap1 = CompleteMaxHeap::from(MyVec::from(data1));
            let heap2 = CompleteMaxHeap::from(MyVec::from(data2));
            heap1.merge(heap2);
            while !heap1.is_empty() {
                let max = heap1.vec.iter().max().copied();
                assert_eq!(heap1.delete_max(), max);
            }
        }
    }
}
