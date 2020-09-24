use super::List;
use crate::vec::MyVec;

impl<T: List<isize>> ISizeListExt for T {}

pub trait ISizeListExt: List<isize> {
    /// 计算最小的未出现的正整数.
    /// ([-5, 3, 2, 3] => 1; [1, 2, 3] => 4)
    /// # 算法
    /// 注意到最小未出现的正整数的最大可能值为`len + 1`, 故可使用
    /// 大小为`len`的bitmap来表示已出现的正整数集合, 只需扫描一遍
    /// 并分析bitmap即可找到最小未出现的正整数.(第一个不在集合中的
    /// 正整数, 若全都在, 则答案必为`len + 1`).
    // 习题 2.13
    fn smallest_missing_positive_integer(&self) -> isize {
        if self.is_empty() {
            1
        } else {
            let mut bitmap = MyVec::new();
            let len = self.len();
            for i in 0..len {
                bitmap.insert(i, false).unwrap();
            }
            for i in 0..self.len() {
                let item = *self.get(i).unwrap();
                if 0 < item && (item as usize) <= self.len() {
                    *bitmap.get_mut(item as usize - 1).unwrap() = true;
                }
            }
            for i in 0..len {
                if !*bitmap.get(i).unwrap() {
                    return (i + 1) as isize;
                }
            }
            (len + 1) as isize
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::vec::MyVec;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_smallest_missing_positive_integer(data: Vec<isize>) {
            let mut list = MyVec::new();
            for v in data {
                list.insert(list.len(), v).unwrap()
            }
            let res = list.smallest_missing_positive_integer();
            prop_assert!(res > 0);
            let mut bitmap = vec![false; res as usize - 1];
            for i in 0..list.len() {
                let x = *list.get(i).unwrap();
                prop_assert!(x != res);
                if 0 < x && x < res {
                    bitmap[x as usize - 1] = true;
                }
            }
            prop_assert!(bitmap.iter().all(|v| *v));
        }
    }
}
