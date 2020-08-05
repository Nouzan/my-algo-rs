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

    /// 逆置所有元素.
    fn reverse(&mut self) {
        let (mut i, mut j) = (0, self.len() - 1);
        while i < j {
            self.swap(i, j).unwrap();
            i += 1;
            j -= 1;
        }
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
    use super::{IndexError, List, PartialEqListExt, PartialOrdListExt};

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

        x.sort();

        for (idx, v) in vec![1, 1, 1, 2, 3, 5, 6, 7, 7, 9, 11, 11]
            .iter()
            .enumerate()
        {
            assert_eq!(*List::get(&x, idx)?, *v);
        }

        Ok(())
    }
}
