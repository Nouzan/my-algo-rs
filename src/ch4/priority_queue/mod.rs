pub mod complete_heap;

use crate::vec::MyVec;

/// 最大优先队列.
pub trait PriorityQueue<T: PartialOrd>: From<MyVec<T>> {
    /// 获取队列长度.
    fn len(&self) -> usize;

    /// 队列是否为空.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 删除最大元素.
    /// 若队列为空则返回`None`.
    fn delete_max(&mut self) -> Option<T>;

    /// 读取最大元素.
    /// 若队列为空则返回`None`.
    fn get_max(&self) -> Option<&T>;

    /// 向队列插入一个元素.
    fn insert(&mut self, elem: T);
}
