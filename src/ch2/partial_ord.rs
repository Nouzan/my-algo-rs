use super::{IndexError, List};
use std::ops::{Deref, DerefMut};

impl<T, U> PartialOrdListExt<U> for T
where
    T: List<U>,
    U: PartialOrd,
{
}

/// `List` trait的一个扩展trait, 提供了一些基于偏序的方法.
pub trait PartialOrdListExt<Item: PartialOrd>: List<Item> {
    /// 二分搜索. 在有序表中搜索元素`x`, 返回最大的不超过`x`的元素序号, 若所有元素都比`x`大, 则返回`None`.
    /// # Correnctness
    /// 此方法要求表有序(且为顺序).
    fn search(&self, x: &Item) -> Option<usize> {
        if self.is_empty() {
            None
        } else {
            let (mut begin, mut end) = (0, self.len() - 1);
            let a = self.get(begin).unwrap();
            let b = self.get(end).unwrap();
            if *a <= *x && *x < *b {
                // 上式即为循环不变式.
                while begin + 1 < end {
                    // 若`begin` + 1 < `end`, 则`mid`必然不等于`begin`或`end`,
                    // 因此每一次迭代后, `begin`与`end`必然会更接近, 故循环会终止.
                    // 而循环终止于`begin` == `end` 或 `begin` + 1 == `end`,
                    // 不论何种情况, `begin`都是目标结果.
                    // (事实上由论证的第一行, 不可能有`begin` == `end`.)
                    let mid = (begin + end) / 2;
                    let c = self.get(mid).unwrap();
                    if *c <= *x {
                        begin = mid;
                    } else {
                        end = mid;
                    }
                }
                Some(begin)
            } else if *x < *a {
                None
            } else {
                Some(self.len() - 1)
            }
        }
    }

    /// 给出两个有序表的中位数(即两个有序表合并为新的有序表后的中位数).
    /// # Correctness
    /// 此方法要求两个表有序且为顺序.
    // 习题 2.11
    fn merge_mid<'a>(&'a self, rhs: &'a Self) -> Option<&'a Item> {
        if self.is_empty() {
            rhs.mid()
        } else if rhs.is_empty() {
            self.mid()
        } else {
            let (mut ia, mut ja) = (0, self.len());
            let (mut ib, mut jb) = (0, rhs.len());
            while (ja - ia) != 0 && (jb - ib) != 0 {
                let ma = self.mid_between(ia, ja).unwrap();
                let mb = rhs.mid_between(ib, jb).unwrap();
                if *ma <= *mb {
                    let idx = ((ja - ia) + (jb - ib) + 1) / 2 - 1;
                    let idy = (ja - ia + 1) / 2 + (jb - ib + 1) / 2 - 2;
                    let la = if idy < idx {
                        (ja - ia + 1) / 2
                    } else {
                        (ja - ia + 1) / 2 - 1
                    };
                    let lb = if idy > idx {
                        (jb - ib) / 2 + 1
                    } else {
                        (jb - ib) / 2
                    };
                    let l = usize::min(la, lb);
                    if l == 0 {
                        break;
                    }
                    ia += l;
                    jb -= l;
                } else {
                    let idx = ((ja - ia) + (jb - ib) + 1) / 2 - 1;
                    let idy = (ja - ia + 1) / 2 + (jb - ib + 1) / 2 - 2;
                    let lb = if idy < idx {
                        (jb - ib + 1) / 2
                    } else {
                        (jb - ib + 1) / 2 - 1
                    };
                    let la = if idy > idx {
                        (ja - ia) / 2 + 1
                    } else {
                        (ja - ia) / 2
                    };
                    let l = usize::min(la, lb);
                    if l == 0 {
                        break;
                    }
                    ib += l;
                    ja -= l;
                }
            }
            let ma = self.mid_between(ia, ja);
            let mb = rhs.mid_between(ib, jb);
            if let (Some(ma), Some(mb)) = (ma, mb) {
                if (ja - ia) == 1 && (jb - ib) == 1 {
                    if *ma <= *mb {
                        Some(ma)
                    } else {
                        Some(mb)
                    }
                } else if (ja - ia) == 1 {
                    if *ma <= *mb {
                        if (jb - ib) % 2 == 0 {
                            Some(mb)
                        } else {
                            let idx = (jb + ib + 1) / 2 - 2;
                            let c = rhs.get(idx).unwrap();
                            if *ma <= *c {
                                Some(c)
                            } else {
                                Some(ma)
                            }
                        }
                    } else if (jb - ib) % 2 == 1 {
                        Some(mb)
                    } else {
                        let idx = (jb + ib + 1) / 2;
                        let c = rhs.get(idx).unwrap();
                        if *c < *ma {
                            Some(c)
                        } else {
                            Some(ma)
                        }
                    }
                } else if *mb < *ma {
                    if (ja - ia) % 2 == 0 {
                        Some(ma)
                    } else {
                        let idx = (ja + ia + 1) / 2 - 2;
                        let c = self.get(idx).unwrap();
                        if *mb < *c {
                            Some(c)
                        } else {
                            Some(mb)
                        }
                    }
                } else if (ja - ia) % 2 == 1 {
                    Some(ma)
                } else {
                    let idx = (ja + ia + 1) / 2;
                    let c = self.get(idx).unwrap();
                    if *c <= *mb {
                        Some(c)
                    } else {
                        Some(mb)
                    }
                }
            } else if ma.is_some() {
                ma
            } else {
                mb
            }
        }
    }

    /// 给出有序表的中位数(第`⌈len / 2⌉`个).
    /// # Correctness
    /// 此方法要求表有序(且为顺序).
    fn mid(&self) -> Option<&Item> {
        self.mid_between(0, self.len())
    }

    /// 在[a, b)范围内计算中位数(第`⌈(a + b) / 2⌉`个).
    /// # Correctness
    /// 此方法要求表有序(且为顺序).
    fn mid_between(&self, a: usize, b: usize) -> Option<&Item> {
        if b <= a || !self.is_index_read_valid(a) || !self.is_index_insert_valid(b) {
            None
        } else {
            let idx = (a + b + 1) / 2 - 1; // ⌊(a + b + 1) / 2⌋ == ⌈(a + b) / 2⌉
            Some(self.get(idx).unwrap())
        }
    }

    /// 合并两个有序表, 得到一个新的有序表.
    /// # Correctness
    /// 此方法要求表有序(且为顺序).
    /// # 算法
    /// 从头部开始删除元素(先行逆置以避免不必要的移位), 将较小者插入新的表, 直到两个表都为空.
    /// 该算法的时间复杂度为`O(n_1 + n_2)`, 空间复杂度为`O(n_1 + n_2)`.
    // 习题 2.7
    fn merge(mut self, mut rhs: Self) -> Self
    where
        Self: Sized + Default,
    {
        let mut res = Self::default();
        self.reverse();
        rhs.reverse();
        while !self.is_empty() && !rhs.is_empty() {
            let l = self.get(self.len() - 1).unwrap();
            let r = rhs.get(rhs.len() - 1).unwrap();

            // 这里用到了顺序性
            let x = if *l < *r {
                self.delete(self.len() - 1).unwrap()
            } else {
                rhs.delete(rhs.len() - 1).unwrap()
            };
            res.insert(res.len(), x).unwrap();
        }
        let mut remain = if !self.is_empty() { self } else { rhs };
        while !remain.is_empty() {
            res.insert(res.len(), remain.delete(remain.len() - 1).unwrap())
                .unwrap();
        }
        res
    }

    /// 对有序表去重. 这是一个保序的算法.
    /// # Correctness
    /// 此方法要求表有序(且为顺序).
    fn dedup_sorted(&mut self) {
        if !self.is_empty() {
            let mut k: usize = 0; // k始终指向已知最后一个不重复的元素
            for i in 1..self.len() {
                let x = self.get(k).unwrap();
                let y = self.get(i).unwrap();
                if *x != *y {
                    k += 1;
                    self.swap(k, i).unwrap();
                }
            }
            k += 1;
            for i in (k..self.len()).rev() {
                self.delete(i).unwrap();
            }
        }
    }

    /// 删除表中值介于`x`, `y`之间(含)的所有元素(`x` < `y`), 返回被删除的元素列表. 这是一个保序的算法, 但返回的列表并不保持顺序.
    /// 注意: 该方法与`delete_between`不同, 不要求表是有序的.
    fn delete_between_unsorted(&mut self, x: &Item, y: &Item) -> Vec<Item> {
        if self.is_empty() || *x >= *y {
            vec![]
        } else {
            let mut k: Option<usize> = None; // 始终指向已知最后一个不需要删除的元素
            for i in 0..self.len() {
                let z = self.get(i).unwrap();
                if *z < *x || *y < *z {
                    k = k.map_or(Some(0), |v| Some(v + 1));
                    self.swap(i, k.unwrap()).unwrap();
                }
            }
            let k = k.map_or(0, |v| v + 1);
            let mut result = vec![];
            for i in (k..self.len()).rev() {
                result.push(self.delete(i).unwrap());
            }
            result
        }
    }

    /// 删除有序表中值介于`x`,`y`之间的所有元素(`x` < `y`, 不含`x`及`y`), 返回被删除的元素列表. 这是一个保序的算法, 但返回的列表并不保持顺序.
    /// # Correctness
    /// 此方法要求表有序(且为顺序).
    fn delete_between(&mut self, x: &Item, y: &Item) -> Vec<Item> {
        self.delete_between_opt(x, y, false)
    }

    /// 删除有序表中值介于`x`,`y`之间的所有元素(`x` < `y`, 若`contain`为`true`则包含`x`及`y`), 返回被删除的元素列表. 这是一个保序的算法, 但返回的列表并不保持顺序.
    /// # Correctness
    /// 此方法要求表有序(且为顺序).
    ///
    /// 算法的正确性依赖于交换循环的不变式: `j`始终指向首个需要被删除的元素, `k`始终指向首个不需要被删除的元素.
    fn delete_between_opt(&mut self, x: &Item, y: &Item, contain: bool) -> Vec<Item> {
        if self.is_empty() || *x >= *y {
            vec![]
        } else {
            let (mut j, mut k) = (None, None);
            if contain {
                for i in 0..self.len() {
                    let z = self.get(i).unwrap();
                    if *z < *x {
                        j = Some(i);
                    }
                    if *z <= *y {
                        k = Some(i);
                    } else {
                        break;
                    }
                }
            } else {
                for i in 0..self.len() {
                    let z = self.get(i).unwrap();
                    if *z <= *x {
                        j = Some(i);
                    }
                    if *z < *y {
                        k = Some(i);
                    } else {
                        break;
                    }
                }
            }
            let mut j = j.map_or(0, |v| v + 1);
            let mut k = k.map_or(0, |v| v + 1);
            if j == k {
                vec![]
            } else {
                while k < self.len() {
                    self.swap(j, k).unwrap();
                    j += 1;
                    k += 1;
                }
                let mut result = Vec::new();
                for i in (j..self.len()).rev() {
                    let item = self.delete(i).unwrap();
                    result.push(item);
                }
                result
            }
        }
    }

    /// 快速排序中的helper function.
    /// # Panics
    /// 越界时报错.
    fn partition(&mut self, begin: usize, end: usize) -> Option<usize> {
        let mut k = None;
        for i in begin..end {
            let x = self.get(end).unwrap();
            let y = self.get(i).unwrap();
            if *y < *x {
                k = k.map_or(Some(begin), |v| Some(v + 1));
                self.swap(i, k.unwrap()).unwrap();
            }
        }
        k = k.map_or(Some(begin), |v| Some(v + 1));
        self.swap(end, k.unwrap()).unwrap();
        k
    }

    /// 对`begin`~`end`之间的元素进行快速排序.
    fn sort_between(&mut self, begin: usize, end: usize) {
        if begin < end && self.is_index_read_valid(begin) && self.is_index_read_valid(end) {
            let k = self.partition(begin, end).unwrap(); // 这里一定是一个Some.
            if k > begin {
                self.sort_between(begin, k - 1);
            }
            self.sort_between(k + 1, end);
        }
    }

    /// 快速排序.
    fn sort(&mut self) {
        if !self.is_empty() {
            self.sort_between(0, self.len() - 1);
        }
    }

    /// 寻找第一个极小元的位置. 若表空, 则返回`None`.
    fn locate_min(&self) -> Option<usize> {
        if !self.is_empty() {
            let mut min = 0;
            for i in 1..self.len() {
                let x = self.get(i).unwrap();
                let y = self.get(min).unwrap();
                if *x < *y {
                    min = i;
                }
            }
            Some(min)
        } else {
            None
        }
    }

    /// 删除第一个极小元. 若表空, 则返回`None`. 这不是一个保序的算法.
    fn delete_min(&mut self) -> Option<Item> {
        self.locate_min().map(|idx| {
            self.swap(idx, self.len() - 1).unwrap();
            self.delete(self.len() - 1).unwrap()
        })
    }
}

impl<T> List<T> for Vec<T> {
    fn len(&self) -> usize {
        self.len()
    }

    fn get(&self, index: usize) -> Result<&T, IndexError> {
        if self.is_index_read_valid(index) {
            Ok((self as &dyn Deref<Target = [T]>).get(index).unwrap())
        } else {
            Err(IndexError {})
        }
    }

    fn get_mut(&mut self, index: usize) -> Result<&mut T, IndexError> {
        if self.is_index_read_valid(index) {
            Ok((self as &mut dyn DerefMut<Target = [T]>)
                .get_mut(index)
                .unwrap())
        } else {
            Err(IndexError {})
        }
    }

    fn swap(&mut self, i: usize, j: usize) -> Result<(), IndexError> {
        if i == j {
            Ok(())
        } else if self.is_index_read_valid(i) && self.is_index_read_valid(j) {
            (self as &mut dyn DerefMut<Target = [T]>).swap(i, j);
            Ok(())
        } else {
            Err(IndexError {})
        }
    }

    fn insert(&mut self, index: usize, x: T) -> Result<(), IndexError> {
        if self.is_index_insert_valid(index) {
            self.push(x);
            let mut i = self.len() - 1;
            while i != index {
                (self as &mut dyn DerefMut<Target = [T]>).swap(i - 1, i);
                i -= 1;
            }
            Ok(())
        } else {
            Err(IndexError {})
        }
    }

    fn delete(&mut self, index: usize) -> Result<T, IndexError> {
        if self.is_index_read_valid(index) {
            let mut i = index;
            let last = self.len() - 1;
            while i != last {
                (self as &mut dyn DerefMut<Target = [T]>).swap(i, i + 1);
                i += 1;
            }
            Ok(self.pop().unwrap())
        } else {
            Err(IndexError {})
        }
    }
}
