use super::List;

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

    /// 计算序列的主元素, 若存在则返回该元素的引用, 若不存在则返回`None`.
    /// 一个序列的主元素定义为其中出现次数超过`len/2`的元素.
    /// # 算法
    /// 从左往右扫描. `c`初始化为1, `x`初始化为第1个元素, 后面每遇到一个元素,
    /// 若与`x`相同则`c+=1`, 若不同则`c-=1`, 若`c`归0则重新初始化为1, 并
    /// 将x改为当前元素, 直到扫描完毕. 则`x`为主元素的唯一候选, 此时只需要再扫描
    /// 一遍, 检验`x`是否为主元素即可.
    // 习题2.12
    fn primary(&self) -> Option<&Item> {
        if self.is_empty() {
            None
        } else {
            let mut x = self.get(0).unwrap();
            let mut count: usize = 1;
            for i in 1..self.len() {
                let y = self.get(i).unwrap();
                if *x == *y {
                    count += 1;
                } else {
                    count -= 1;
                }
                if count == 0 {
                    count = 1;
                    x = y;
                }
            }
            count = 0;
            for i in 0..self.len() {
                let y = self.get(i).unwrap();
                if *x == *y {
                    count += 1;
                }
            }
            if count > self.len() / 2 {
                Some(x)
            } else {
                None
            }
        }
    }
}
