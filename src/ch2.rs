use std::ops::{Deref, DerefMut};

/// 下标错误类型.
#[derive(Debug)]
pub struct IndexError;

/// 元素类型为`Item`的线性表.
pub trait List<Item>: Default {
    /// 判断线性表是否为空.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 判断下标是否越界(访问时).
    fn is_index_read_valid(&self, index: usize) -> bool {
        index < self.len()
    }

    /// 判断下标是否越界(插入时).
    fn is_index_insert_valid(&self, index: usize) -> bool {
        index <= self.len()
    }

    /// 交换`i`, `j`两个位置的元素.
    fn swap(&mut self, i: usize, j: usize) -> Result<(), IndexError>;

    /// 逆置[begin, end]之间的元素.
    fn reverse_between(&mut self, begin: usize, end: usize) -> Result<(), IndexError> {
        if begin <= end && self.is_index_read_valid(begin) && self.is_index_read_valid(end) {
            let (mut i, mut j) = (begin, end);
            while i < j {
                self.swap(i, j).unwrap();
                i += 1;
                j -= 1;
            }
            Ok(())
        } else {
            Err(IndexError {})
        }
    }

    /// 逆置所有元素.
    fn reverse(&mut self) {
        self.reverse_between(0, self.len() - 1).unwrap();
    }

    /// 获取线性表的长度.
    fn len(&self) -> usize;

    /// 获取序号为`index`的元素的只读引用.
    ///
    /// # Errors
    ///
    /// 若位置不合法, 返回错误.
    fn get(&self, index: usize) -> Result<&Item, IndexError>;

    /// 获取序号为`index`的元素的可写引用.
    ///
    /// # Errors
    ///
    /// 若位置不合法, 返回错误.
    fn get_mut(&mut self, index: usize) -> Result<&mut Item, IndexError>;

    /// 在位置`index`插入元素. 新元素将会被放置在位置`index`, 原来`index`位置及其后元素后移1位.
    ///
    /// # Errors
    ///
    /// 若位置不合法, 返回错误.
    fn insert(&mut self, index: usize, x: Item) -> Result<(), IndexError>;

    /// 删除位置`index`上的元素. 其后元素将会前移1位置, 填补空缺.
    ///
    /// # Errors
    ///
    /// 若位置不合法, 返回错误.
    fn delete(&mut self, index: usize) -> Result<Item, IndexError>;

    /// 将`List<T>`转化为`List<&T>`
    fn to_refs<'a, T>(&'a self) -> T
    where
        T: List<&'a Item>,
        Item: 'a,
    {
        let mut res = T::default();
        for i in 0..self.len() {
            res.insert(i, self.get(i).unwrap()).unwrap();
        }
        res
    }
}

impl<T, U> ListExt<U> for T where T: List<U> {}

/// `List` trait的一个扩展trait.
pub trait ListExt<Item>: List<Item> {
    /// 循环左移`index`个位置.
    /// # 算法
    /// 实际上我们只需要将`[0, index)`与`[index, len)`位置上的进行交换, 即交换后变为`[index, len), [0, index)`.
    /// 而这个过程可以通过先对所有元素逆置, 再在分割位置前后各自逆置完成, 如:
    /// `[0, 1, 2, 3, 4, 5] -> [5, 4, 3, 2, 1] -> [4, 5, 1, 2, 3]`.
    /// 它的思想可类比于给定`ab`求`ba`, 而`(a^{-1}b^{-1})^{-1} == ba`(串运算).
    /// 这个算法的时间复杂度为`O(n)`, 空间复杂度为`O(1)`.
    // 习题 2.8, 2.10
    fn shift(&mut self, index: usize) -> Result<(), IndexError> {
        if self.is_index_read_valid(index) {
            let split = self.len() - index;
            self.reverse();
            self.reverse_between(0, split - 1).unwrap();
            self.reverse_between(split, self.len() - 1).unwrap();
            Ok(())
        } else {
            Err(IndexError {})
        }
    }
}

impl<T, U> PartialEqListExt<U> for T
where
    T: List<U>,
    U: PartialEq,
{
}

/// `List` trait的一个扩展trait, 提供了一些基于判等的方法.
pub trait PartialEqListExt<Item: PartialEq>: List<Item> {
    /// 查找值等于`x`的元素, 找到后返回序号, 若未找到则返回`None`.
    fn locate(&self, x: &Item) -> Option<usize> {
        for i in 0..self.len() {
            if *x == *self.get(i).unwrap() {
                return Some(i);
            }
        }
        None
    }

    /// 删除所有值等于`x`的元素. 这是一个不保序的算法.
    fn delete_all(&mut self, x: &Item) {
        for i in (0..self.len()).rev() {
            if *self.get(i).unwrap() == *x {
                self.swap(i, self.len() - 1).unwrap();
                self.delete(self.len() - 1).unwrap();
            }
        }
    }
}

impl<T, U> PartialOrdListExt<U> for T
where
    T: List<U>,
    U: PartialOrd,
{
}

/// `List` trait的一个扩展trait, 提供了一些基于偏序的方法.
pub trait PartialOrdListExt<Item: PartialOrd>: PartialEqListExt<Item> {
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
        Self: Sized,
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

#[cfg(test)]
mod test {
    use super::{IndexError, List, ListExt, PartialEqListExt, PartialOrdListExt};
    use crate::vec::MyVec;
    use proptest::prelude::*;

    #[test]
    fn test_insert() -> Result<(), IndexError> {
        let mut x: MyVec<usize> = MyVec::new();
        assert_eq!(List::len(&x), 0);
        assert!(List::is_empty(&x));
        List::insert(&mut x, 0, 11)?;
        assert_eq!(*List::get(&x, 0)?, 11);
        assert_eq!(List::len(&x), 1);
        List::insert(&mut x, 0, 12)?;
        assert_eq!(List::len(&x), 2);
        assert_eq!(*List::get(&x, 1)?, 11);
        assert_eq!(*List::get(&x, 0)?, 12);
        Ok(())
    }

    #[test]
    fn test_delete() -> Result<(), IndexError> {
        let mut x: MyVec<usize> = MyVec::new();
        List::insert(&mut x, 0, 11)?;
        assert_eq!(*List::get(&x, 0)?, 11);
        List::insert(&mut x, 0, 12)?;
        assert_eq!(*List::get(&x, 1)?, 11);
        assert_eq!(*List::get(&x, 0)?, 12);
        List::delete(&mut x, 0)?;
        assert_eq!(List::len(&x), 1);
        assert_eq!(*List::get(&x, 0)?, 11);
        List::delete(&mut x, 0)?;
        assert_eq!(List::len(&x), 0);
        Ok(())
    }

    #[test]
    fn test_locate() -> Result<(), IndexError> {
        let mut x: MyVec<usize> = MyVec::new();
        List::insert(&mut x, 0, 11)?;
        assert_eq!(PartialEqListExt::locate(&x, &11), Some(0));
        Ok(())
    }

    #[test]
    fn test_locate_min() -> Result<(), IndexError> {
        let mut x: MyVec<usize> = MyVec::new();
        List::insert(&mut x, 0, 11)?;
        assert_eq!(x.locate_min(), Some(0));
        List::insert(&mut x, 0, 10)?;
        assert_eq!(x.locate_min(), Some(0));
        List::insert(&mut x, 2, 9)?;
        assert_eq!(x.locate_min(), Some(2));
        Ok(())
    }

    #[test]
    fn test_delete_min() -> Result<(), IndexError> {
        let mut x: MyVec<usize> = MyVec::new();
        List::insert(&mut x, 0, 11)?;
        List::insert(&mut x, 0, 10)?;
        List::insert(&mut x, 2, 9)?;
        assert_eq!(x.delete_min(), Some(9));
        assert_eq!(List::len(&x), 2);
        Ok(())
    }

    #[test]
    fn test_reverse() -> Result<(), IndexError> {
        let mut x: MyVec<usize> = MyVec::new();
        List::insert(&mut x, 0, 11)?;
        List::insert(&mut x, 0, 10)?;
        List::insert(&mut x, 2, 9)?;
        // before: 10, 11, 9
        assert_eq!(*List::get(&x, 0)?, 10);
        assert_eq!(*List::get(&x, 1)?, 11);
        assert_eq!(*List::get(&x, 2)?, 9);
        List::reverse(&mut x);
        // after: 9, 11, 10
        assert_eq!(*List::get(&x, 0)?, 9);
        assert_eq!(*List::get(&x, 1)?, 11);
        assert_eq!(*List::get(&x, 2)?, 10);
        List::insert(&mut x, 0, 12)?;
        // before: 12, 9, 11, 10
        List::reverse(&mut x);
        // after: 10, 11, 9, 12
        assert_eq!(*List::get(&x, 0)?, 10);
        assert_eq!(*List::get(&x, 1)?, 11);
        assert_eq!(*List::get(&x, 2)?, 9);
        assert_eq!(*List::get(&x, 3)?, 12);
        Ok(())
    }

    proptest! {
        #[test]
        fn test_delete_all(data: Vec<usize>) {
            let mut x: MyVec<usize> = MyVec::new();
            for v in data.iter() {
                let len = x.len();
                List::insert(&mut x, len, *v).unwrap();
            }

            if !x.is_empty() {
                let v = *x.get(0).unwrap();
                x.delete_all(&v);
                for i in 0..x.len() {
                    let w = x.get(i).unwrap();
                    prop_assert_ne!(v, *w);
                }
            }
        }
    }

    proptest! {
        #[test]
        fn test_sort(data: Vec<usize>) {
            let mut x: MyVec<usize> = MyVec::new();
            for v in data.iter() {
                let len = x.len();
                List::insert(&mut x, len, *v).unwrap();
            }
            x.sort();
            if !x.is_empty() {
                let mut last = x.get(0).unwrap();
                for i in 1..x.len() {
                    let now = x.get(i).unwrap();
                    prop_assert!(*last <= *now);
                    last = now;
                }
            }
        }
    }

    #[test]
    fn test_delete_between() -> Result<(), IndexError> {
        let mut x: MyVec<usize> = MyVec::new();
        let res = x.delete_between(&1, &2);
        assert_eq!(res, vec![]);
        for v in vec![7, 1, 9, 11, 2, 3, 1, 5, 7, 11, 1, 6].iter() {
            let len = x.len();
            List::insert(&mut x, len, *v)?;
        }
        x.sort();
        let res = x.delete_between(&7, &4);
        assert_eq!(res, vec![]);
        let mut res = x.delete_between(&3, &9);
        res.sort();
        for (idx, v) in vec![5, 6, 7, 7].iter().enumerate() {
            assert_eq!(*List::get(&res, idx)?, *v);
        }
        for (idx, v) in vec![1, 1, 1, 2, 3, 9, 11, 11].iter().enumerate() {
            assert_eq!(*List::get(&x, idx)?, *v);
        }
        let mut res = x.delete_between(&0, &100);
        res.sort();
        assert_eq!(res, vec![1, 1, 1, 2, 3, 9, 11, 11]);
        assert!(x.is_empty());

        Ok(())
    }

    #[test]
    fn test_delete_between_opt() -> Result<(), IndexError> {
        let mut x: MyVec<usize> = MyVec::new();
        let res = x.delete_between_opt(&1, &2, true);
        assert_eq!(res, vec![]);
        for v in vec![7, 1, 9, 11, 2, 3, 1, 5, 7, 11, 1, 6].iter() {
            let len = x.len();
            List::insert(&mut x, len, *v)?;
        }
        x.sort();
        let res = x.delete_between_opt(&7, &4, true);
        assert_eq!(res, vec![]);
        let mut res = x.delete_between_opt(&3, &9, true);
        res.sort();
        for (idx, v) in vec![3, 5, 6, 7, 7, 9].iter().enumerate() {
            assert_eq!(*List::get(&res, idx)?, *v);
        }
        for (idx, v) in vec![1, 1, 1, 2, 11, 11].iter().enumerate() {
            assert_eq!(*List::get(&x, idx)?, *v);
        }
        let mut res = x.delete_between_opt(&0, &11, true);
        res.sort();
        assert_eq!(res, vec![1, 1, 1, 2, 11, 11]);
        assert!(x.is_empty());

        Ok(())
    }

    #[test]
    fn test_delete_between_unsorted_sorted() -> Result<(), IndexError> {
        let mut x: MyVec<usize> = MyVec::new();
        let res = x.delete_between_unsorted(&1, &2);
        assert_eq!(res, vec![]);
        for v in vec![7, 1, 9, 11, 2, 3, 1, 5, 7, 11, 1, 6].iter() {
            let len = x.len();
            List::insert(&mut x, len, *v)?;
        }
        x.sort();
        let res = x.delete_between_unsorted(&7, &4);
        assert_eq!(res, vec![]);
        let mut res = x.delete_between_unsorted(&3, &9);
        res.sort();
        for (idx, v) in vec![3, 5, 6, 7, 7, 9].iter().enumerate() {
            assert_eq!(*List::get(&res, idx)?, *v);
        }
        for (idx, v) in vec![1, 1, 1, 2, 11, 11].iter().enumerate() {
            assert_eq!(*List::get(&x, idx)?, *v);
        }
        let mut res = x.delete_between_unsorted(&0, &11);
        res.sort();
        assert_eq!(res, vec![1, 1, 1, 2, 11, 11]);
        assert!(x.is_empty());

        Ok(())
    }

    #[test]
    fn test_delete_between_unsorted_unsorted() -> Result<(), IndexError> {
        let mut x: MyVec<usize> = MyVec::new();
        for v in vec![7, 1, 9, 11, 2, 3, 1, 5, 7, 11, 1, 6].iter() {
            let len = x.len();
            List::insert(&mut x, len, *v)?;
        }
        let mut res = x.delete_between_unsorted(&3, &9);
        res.sort();
        for (idx, v) in vec![3, 5, 6, 7, 7, 9].iter().enumerate() {
            assert_eq!(*List::get(&res, idx)?, *v);
        }
        for (idx, v) in vec![1, 11, 2, 1, 11, 1].iter().enumerate() {
            assert_eq!(*List::get(&x, idx)?, *v);
        }
        let mut res = x.delete_between_unsorted(&0, &11);
        res.sort();
        assert_eq!(res, vec![1, 1, 1, 2, 11, 11]);
        assert!(x.is_empty());

        Ok(())
    }

    #[test]
    fn test_dedup_sorted() -> Result<(), IndexError> {
        let mut x: MyVec<usize> = MyVec::new();
        for v in vec![7, 1, 9, 11, 2, 3, 1, 5, 7, 11, 1, 6].iter() {
            let len = x.len();
            List::insert(&mut x, len, *v)?;
        }
        x.sort();
        x.dedup_sorted();
        assert_eq!(*x, *vec![1, 2, 3, 5, 6, 7, 9, 11]);
        Ok(())
    }

    #[test]
    fn test_merge() -> Result<(), IndexError> {
        let mut x: MyVec<usize> = MyVec::new();
        for i in &[1, 3, 5, 6, 8, 10] {
            x.push(*i)
        }
        let mut y: MyVec<usize> = MyVec::new();
        for i in &[2, 4, 6, 7, 9, 11] {
            y.push(*i)
        }
        let z = x.merge(y);
        assert_eq!(*z, *vec![1, 2, 3, 4, 5, 6, 6, 7, 8, 9, 10, 11]);
        Ok(())
    }

    #[test]
    fn test_shift() -> Result<(), IndexError> {
        let mut x: MyVec<usize> = MyVec::new();
        for i in &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
            x.push(*i);
        }
        x.shift(4)?;
        assert_eq!(*x, *vec![4, 5, 6, 7, 8, 9, 0, 1, 2, 3]);
        Ok(())
    }

    #[test]
    fn test_search() -> Result<(), IndexError> {
        let mut x: MyVec<usize> = MyVec::new();
        for i in &[1, 2, 3, 3, 3, 4, 5, 6, 9, 11] {
            x.push(*i);
        }
        assert_eq!(x.search(&3), Some(4));
        assert_eq!(x.search(&4), Some(5));
        assert_eq!(x.search(&0), None);
        assert_eq!(x.search(&12), Some(9));
        assert_eq!(x.search(&1), Some(0));
        assert_eq!(x.search(&11), Some(9));
        let mut x: MyVec<usize> = MyVec::new();
        x.push(1);
        assert_eq!(x.search(&1), Some(0));
        assert_eq!(x.search(&0), None);
        assert_eq!(x.search(&11), Some(0));
        Ok(())
    }

    #[test]
    fn test_mid() -> Result<(), IndexError> {
        let mut x: MyVec<usize> = MyVec::new();
        for i in &[11, 13, 15, 17, 19] {
            x.push(*i);
        }
        assert_eq!(x.mid(), Some(&15));

        Ok(())
    }

    proptest! {
        #[test]
        fn test_merge_mid(a: Vec<isize>, b: Vec<isize>) {
            let mut x: MyVec<isize> = MyVec::new();
            let mut y: MyVec<isize> = MyVec::new();
            let mut z: MyVec<isize> = MyVec::new();
            for v in a.iter() {
                let len = x.len();
                List::insert(&mut x, len, *v).unwrap();
                let len = z.len();
                List::insert(&mut z, len, *v).unwrap();
            }
            for v in b.iter() {
                let len = y.len();
                List::insert(&mut y, len, *v).unwrap();
                let len = z.len();
                List::insert(&mut z, len, *v).unwrap();
            }
            x.sort();
            y.sort();
            z.sort();
            let mid = x.merge_mid(&y).map(|v| *v);
            let zmid = z.mid().map(|v| *v);
            assert_eq!(mid, zmid);
        }
    }

    #[test]
    fn test_merge_mid_debug() {
        let mut x: MyVec<isize> = MyVec::new();
        let mut y: MyVec<isize> = MyVec::new();
        let mut z: MyVec<isize> = MyVec::new();
        let a = vec![
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            -2897281253,
            -6196200173237906148,
            8925592336651969408,
            2455274111052877208,
            -656826015673857378,
            -3918187044768650842,
            -7166999091199384584,
            -5996361932006152609,
            2889077960617313680,
            88271664092555393,
            2899996647040923106,
            -6516672597673576215,
            -5546300437270337926,
            1473724560447567648,
            2059516156938740199,
            -171858695312451639,
            8522714349128348863,
            4264948759076353221,
            1594454708689104818,
            7681715165400207596,
            -8148621855136367066,
            -644209070338910506,
            5220468932815270648,
            -6625493688883664448,
            1397876726542150767,
            -4727728100751890483,
            7531905508060828888,
            6519918035677179251,
            -5583427334731038398,
            -2967049700757160626,
            -1721576352507059971,
            1727757225621534506,
            2170186811265331760,
            5922347655179323346,
            -8204506787910346843,
            3005523099967943604,
            -1605301380642725877,
            -7332893205306610347,
            -938746561832165523,
            -8789972477573055083,
            -3835164436479195405,
            4786210369317527761,
            3594225956822071679,
            -1748670941390812505,
            3876800443306823381,
            -2257341117945605237,
            5677367518449984234,
            8405782022434682455,
            7581027924183879849,
            4096207420437071452,
            8085920046616710860,
        ];
        let b = vec![
            7482600966255041402,
            -8333722760038280943,
            -4250333620945924187,
            -880643609219279756,
            -5071519582309135839,
            -3223679753249750427,
            99164523170473582,
            3290501520063790669,
            -4598488739711737148,
            4473989299141740021,
            -6781163372128545589,
            5548457122780486112,
            -3557150876905369710,
            -6908408383691144508,
            -6691672864717401851,
            -1937234497355224888,
            1707928323010534440,
            -6339963453647765820,
            7531816131263962515,
            -1284471083586039299,
            7403650438578929422,
            -3829572531986954543,
            386140615396125578,
            -7203738925830004739,
            -8544999182076961763,
            -1490629782192538174,
            5090487921526136898,
            -5141834306885877895,
            -6956565386351062722,
            -7576159871494786891,
            -7491376982597399724,
            4720093450235912204,
            -4053929147728379618,
            4161325017029619931,
            -7081740323715740893,
            8102254179923436400,
            461968019096134908,
            2689246687889717639,
            -7665274172393783307,
            4662732249364662193,
            70100343326846188,
            6099973236709120471,
            -5341597363607795057,
            -7862231724152292154,
            734124934836851694,
            2449474722449367057,
            3081651409021500712,
            7122530452107911687,
            -6074493196840102323,
            4838248576879314072,
            7383191579363050811,
            -3914598055905828817,
            3109065319387327394,
            -1781297907957027685,
            -3583352287771982849,
        ];
        for v in a.iter() {
            let len = x.len();
            List::insert(&mut x, len, *v).unwrap();
            let len = z.len();
            List::insert(&mut z, len, *v).unwrap();
        }
        for v in b.iter() {
            let len = y.len();
            List::insert(&mut y, len, *v).unwrap();
            let len = z.len();
            List::insert(&mut z, len, *v).unwrap();
        }
        x.sort();
        y.sort();
        z.sort();
        let mid = x.merge_mid(&y).map(|v| *v);
        let zmid = z.mid().map(|v| *v);
        assert_eq!(mid, zmid);
    }
}
