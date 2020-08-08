use std::ops::{Deref, DerefMut};

/// 下标错误类型.
#[derive(Debug)]
pub struct IndexError;

/// 元素类型为`Item`的线性表.
pub trait List<Item> {
    /// 创建一个空的线性表.
    fn new() -> Self;

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
}

impl<T, U> ListExt<U> for T where T: List<U> {}

/// `List` trait的一个扩展trait.
pub trait ListExt<Item>: List<Item> {
    /// 将`[0, index)`与`[index, len)`位置上的进行交换, 即交换后变为`[index, len), [0, index)`.
    fn swap_at(&mut self, index: usize) -> Result<(), IndexError> {
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

    /// 合并两个有序表, 得到一个新的有序表.
    /// # Correctness
    /// 此方法要求表有序(且为顺序).
    fn merge(mut self, mut rhs: Self) -> Self
    where
        Self: Sized,
    {
        let mut res = Self::new();
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
    fn new() -> Self {
        Vec::new()
    }

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

    #[test]
    fn test_insert() -> Result<(), IndexError> {
        let mut x: Vec<usize> = List::new();
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
        let mut x: Vec<usize> = List::new();
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
        let mut x: Vec<usize> = List::new();
        List::insert(&mut x, 0, 11)?;
        assert_eq!(PartialEqListExt::locate(&x, &11), Some(0));
        Ok(())
    }

    #[test]
    fn test_locate_min() -> Result<(), IndexError> {
        let mut x: Vec<usize> = List::new();
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
        let mut x: Vec<usize> = List::new();
        List::insert(&mut x, 0, 11)?;
        List::insert(&mut x, 0, 10)?;
        List::insert(&mut x, 2, 9)?;
        assert_eq!(x.delete_min(), Some(9));
        assert_eq!(List::len(&x), 2);
        Ok(())
    }

    #[test]
    fn test_reverse() -> Result<(), IndexError> {
        let mut x: Vec<usize> = List::new();
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

    #[test]
    fn test_delete_all() -> Result<(), IndexError> {
        let mut x: Vec<usize> = List::new();
        for v in vec![7, 1, 9, 11, 2, 3, 1, 5, 7, 11, 1].iter() {
            let len = x.len();
            List::insert(&mut x, len, *v)?;
        }

        x.delete_all(&1);

        for (idx, v) in vec![7, 7, 9, 11, 2, 3, 11, 5].iter().enumerate() {
            assert_eq!(*List::get(&x, idx)?, *v);
        }

        Ok(())
    }

    #[test]
    fn test_sort() -> Result<(), IndexError> {
        let mut x: Vec<usize> = List::new();
        x.sort();
        for v in vec![7, 1, 9, 11, 2, 3, 1, 5, 7, 11, 1, 6].iter() {
            let len = x.len();
            List::insert(&mut x, len, *v)?;
            x.sort();
        }

        x.reverse();
        x.sort();

        for (idx, v) in vec![1, 1, 1, 2, 3, 5, 6, 7, 7, 9, 11, 11]
            .iter()
            .enumerate()
        {
            assert_eq!(*List::get(&x, idx)?, *v);
        }

        Ok(())
    }

    #[test]
    fn test_delete_between() -> Result<(), IndexError> {
        let mut x: Vec<usize> = List::new();
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
        let mut x: Vec<usize> = List::new();
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
        let mut x: Vec<usize> = List::new();
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
        let mut x: Vec<usize> = List::new();
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
        let mut x: Vec<usize> = List::new();
        for v in vec![7, 1, 9, 11, 2, 3, 1, 5, 7, 11, 1, 6].iter() {
            let len = x.len();
            List::insert(&mut x, len, *v)?;
        }
        x.sort();
        x.dedup_sorted();
        assert_eq!(x, vec![1, 2, 3, 5, 6, 7, 9, 11]);
        Ok(())
    }

    #[test]
    fn test_merge() -> Result<(), IndexError> {
        let x: Vec<usize> = vec![1, 3, 5, 6, 8, 10];
        let y: Vec<usize> = vec![2, 4, 6, 7, 9, 11];
        let z = x.merge(y);
        assert_eq!(z, vec![1, 2, 3, 4, 5, 6, 6, 7, 8, 9, 10, 11]);
        Ok(())
    }

    #[test]
    fn test_swap_at() -> Result<(), IndexError> {
        let mut x: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        x.swap_at(4)?;
        assert_eq!(x, vec![4, 5, 6, 7, 8, 9, 0, 1, 2, 3]);
        Ok(())
    }

    #[test]
    fn test_search() -> Result<(), IndexError> {
        let x: Vec<usize> = vec![1, 2, 3, 3, 3, 4, 5, 6, 9, 11];
        assert_eq!(x.search(&3), Some(4));
        assert_eq!(x.search(&4), Some(5));
        assert_eq!(x.search(&0), None);
        assert_eq!(x.search(&12), Some(9));
        assert_eq!(x.search(&1), Some(0));
        assert_eq!(x.search(&11), Some(9));
        let x: Vec<usize> = vec![1];
        assert_eq!(x.search(&1), Some(0));
        assert_eq!(x.search(&0), None);
        assert_eq!(x.search(&11), Some(0));
        Ok(())
    }
}

// 0, 8
// 4, 8
// 4, 6
// 4, 5
// 4, 5
