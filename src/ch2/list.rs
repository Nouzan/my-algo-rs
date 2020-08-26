use super::{Slice, SliceMut};

/// 下标错误类型.
#[derive(Debug)]
pub struct IndexError;

/// 元素类型为`Item`的线性表.
pub trait List<Item> {
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
    fn to_refs<'a, T: Default>(&'a self) -> T
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

    /// 获取一个可变切片范围`start..end`.
    fn slice_mut(
        &mut self,
        start: usize,
        end: usize,
    ) -> Result<SliceMut<'_, Self, Item>, IndexError>
    where
        Self: Sized,
    {
        if start <= end {
            let len = end - start;
            SliceMut::new(self, start, len)
        } else {
            Err(IndexError)
        }
    }

    /// 获取一个只读切片范围`start..end`.
    fn slice(&self, start: usize, end: usize) -> Result<Slice<'_, Self, Item>, IndexError>
    where
        Self: Sized,
    {
        if start <= end {
            let len = end - start;
            Slice::new(self, start, len)
        } else {
            Err(IndexError)
        }
    }
}
