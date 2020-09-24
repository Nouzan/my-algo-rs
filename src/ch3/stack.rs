//! 栈特质与栈算法.
//!
//! ## 栈操作序列与出栈顺序
//! `S`表示入栈, `X`表示出栈.
//!
//! * 定义: 一个 *栈操作序列* 定义为`S`和`X`的一个有限序列.
//!
//! * 定义: 一个 *合法的(admissible)* 的栈操作序列定义为一个`S`与`X`数目相同、且沿着序列执行入栈、出栈操作, 始终不会发生 *下溢(underflow, 栈空时出栈)* 的操作序列.
//!
//! * 命题(*TAOCP2.2.1.3*): 一个栈操作序列是合法的, 当且仅当序列中在任何一个`X`前面的`X`的个数严格小于`S`的个数.
//! ### 命题(*TAOCP2.2.1.3*): 对相同的输入序列, 不同的(相同长度)合法操作序列产生不同的输出序列.
//! **证明**
//! 下面设输入序列充分长.
//! 不妨设合法操作序列`a`, `b`从第`k`个操作开始`a`和`b`不相同, 且`a`中为`S`, `b`中为`X`, 再设`k`个操作以前的输出序列为`{o[1], o[2], ..., o[n]}`.
//! 那么, 在执行第`k`个操作后, `a`的输出序列依然为`{o[1], o[2], ..., o[n]}`, 但此时`b`的输出序列为`{o[1], o[2], ..., o[n], o[n+1]}`.
//! 于是, `a`和`b`的输出序列不可能相同, 因为`a`中要出栈`o[n+1]`(由`b`的合法性假设知, `o[n+1]`必然存在, 且在第`k`个操作前已经入栈), 必须先要出栈第`k`次操作中入栈的元素`x != o[n+1]`,
//! 那么, `a`的输出序列中第`n+1`个元素为`x`与`b`的输出序列中的`n+1`个元素`o[n+1]`不相同.
//!
//! * 推论: 对于长度为`n`的输入序列, `S`和`X`数目分别为`n`的合法操作序列与可能的输出序列一一对应.
//! 因此, 不同的输出序列总数为`C(n) = C(2n, n) / (n + 1)`(*Catalan数*, 也有递推形式: `C(0) = 1, C(n) = C(0)C(n-1) + C(1)C(n-2) + ... + C(n-1)C(0)`)

use super::Queue;
use crate::ch2::{
    linked_list::{cdll, shll, LinearCursor, SinglyLinkedList},
    List,
};
use crate::vec::MyVec;

/// 栈特质.
/// 具有`LIFO`性质.
pub trait Stack {
    /// 栈元素.
    type Elem;

    /// 入栈.
    /// 若入栈成功则返回`None`, 否则返回`item`.
    fn push(&mut self, elem: Self::Elem) -> Option<Self::Elem>;

    /// 出栈.
    /// 若栈空则返回`None`, 否则返回栈顶元素.
    fn pop(&mut self) -> Option<Self::Elem>;

    /// 栈是否为空.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 栈是否满.
    fn is_full(&self) -> bool {
        self.cap().map_or(false, |cap| self.len() == cap)
    }

    /// 栈容量.
    fn cap(&self) -> Option<usize>;

    /// 栈大小.
    fn len(&self) -> usize;

    /// 栈顶元素.
    fn top(&self) -> Option<&Self::Elem>;
}

impl<T> Stack for Vec<T> {
    type Elem = T;

    fn is_full(&self) -> bool {
        false
    }

    fn cap(&self) -> Option<usize> {
        None
    }

    fn len(&self) -> usize {
        List::len(self)
    }

    fn is_empty(&self) -> bool {
        List::is_empty(self)
    }

    fn push(&mut self, item: T) -> Option<T> {
        self.push(item);
        None
    }

    fn pop(&mut self) -> Option<T> {
        self.pop()
    }

    fn top(&self) -> Option<&Self::Elem> {
        self.last()
    }
}

impl<T> Stack for MyVec<T> {
    type Elem = T;

    fn is_full(&self) -> bool {
        false
    }

    fn cap(&self) -> Option<usize> {
        None
    }

    fn len(&self) -> usize {
        List::len(self)
    }

    fn is_empty(&self) -> bool {
        List::is_empty(self)
    }

    fn push(&mut self, item: T) -> Option<T> {
        self.insert(List::len(self), item).unwrap();
        None
    }

    fn pop(&mut self) -> Option<T> {
        if List::is_empty(self) {
            None
        } else {
            self.delete(List::len(self) - 1).ok()
        }
    }

    fn top(&self) -> Option<&Self::Elem> {
        self.last()
    }
}

impl<T> Stack for cdll::LinkedList<T> {
    type Elem = T;

    fn is_full(&self) -> bool {
        false
    }

    fn cap(&self) -> Option<usize> {
        None
    }

    fn len(&self) -> usize {
        SinglyLinkedList::len(self)
    }

    fn is_empty(&self) -> bool {
        SinglyLinkedList::is_empty(self)
    }

    fn push(&mut self, item: T) -> Option<T> {
        self.push_front(item);
        None
    }

    fn pop(&mut self) -> Option<T> {
        if SinglyLinkedList::is_empty(self) {
            None
        } else {
            self.pop_front()
        }
    }

    fn top(&self) -> Option<&Self::Elem> {
        self.cursor_front().into_ref()
    }
}

impl<T> Stack for shll::LinkedList<T> {
    type Elem = T;

    fn is_full(&self) -> bool {
        false
    }

    fn cap(&self) -> Option<usize> {
        None
    }

    fn len(&self) -> usize {
        SinglyLinkedList::len(self)
    }

    fn is_empty(&self) -> bool {
        SinglyLinkedList::is_empty(self)
    }

    fn push(&mut self, item: T) -> Option<T> {
        self.push_front(item);
        None
    }

    fn pop(&mut self) -> Option<T> {
        if SinglyLinkedList::is_empty(self) {
            None
        } else {
            self.pop_front()
        }
    }

    fn top(&self) -> Option<&Self::Elem> {
        self.cursor_front().into_ref()
    }
}

/// 共享栈.
pub struct SharedStack<T> {
    ltop: usize,
    rtop: usize,
    /// 数据容器
    slice: Vec<Option<T>>,
}

impl<T> SharedStack<T> {
    /// 创建一个容量为`size`的共享栈.
    pub fn new(size: usize) -> Self {
        let mut slice = Vec::with_capacity(size);
        for _ in 0..size {
            slice.push(None);
        }
        Self {
            ltop: 0,
            rtop: size,
            slice,
        }
    }

    /// 获取共享栈的总容量.
    pub fn cap(&self) -> usize {
        self.slice.len()
    }

    /// 左栈大小.
    pub fn left_len(&self) -> usize {
        self.ltop
    }

    /// 左栈容量.
    pub fn left_cap(&self) -> usize {
        self.rtop
    }

    /// 左栈是否为空.
    pub fn is_left_empty(&self) -> bool {
        self.left_len() == 0
    }

    /// 左栈是否满.
    pub fn is_left_full(&self) -> bool {
        self.left_len() == self.left_cap()
    }

    /// 右栈大小.
    pub fn right_len(&self) -> usize {
        self.cap() - self.rtop
    }

    /// 右栈容量.
    pub fn right_cap(&self) -> usize {
        self.cap() - self.ltop
    }

    /// 右栈是否为空.
    pub fn is_right_empty(&self) -> bool {
        self.right_len() == 0
    }

    /// 左栈是否满.
    pub fn is_right_full(&self) -> bool {
        self.right_len() == self.right_cap()
    }

    /// 获取左栈.
    pub fn as_left_stack(&mut self) -> LeftStack<T> {
        LeftStack { shared: self }
    }

    /// 获取右栈.
    pub fn as_right_stack(&mut self) -> RightStack<T> {
        RightStack { shared: self }
    }
}

impl<T> Stack for SharedStack<T> {
    type Elem = T;

    fn len(&self) -> usize {
        self.left_len()
    }

    fn cap(&self) -> Option<usize> {
        Some(self.left_cap())
    }

    fn top(&self) -> Option<&Self::Elem> {
        if Stack::is_empty(self) {
            None
        } else {
            self.slice[self.ltop - 1].as_ref()
        }
    }

    fn push(&mut self, elem: Self::Elem) -> Option<Self::Elem> {
        self.as_left_stack().push(elem)
    }

    fn pop(&mut self) -> Option<Self::Elem> {
        self.as_left_stack().pop()
    }
}

impl<T> Queue for SharedStack<T> {
    type Elem = T;

    fn is_empty(&self) -> bool {
        self.is_left_empty() && self.is_right_empty()
    }

    fn is_full(&self) -> bool {
        if self.is_right_empty() {
            self.is_right_full() || self.left_cap() == 0
        } else {
            self.left_len() == self.left_cap().min(self.right_cap())
        }
    }

    fn enqueue(&mut self, elem: Self::Elem) -> Option<Self::Elem> {
        if Queue::is_full(self) {
            Some(elem)
        } else {
            if self.left_len() == self.left_cap().min(self.right_cap())
                && self.as_right_stack().is_empty()
            {
                while let Some(elem) = self.as_left_stack().pop() {
                    self.as_right_stack().push(elem);
                }
            }
            self.as_left_stack().push(elem)
        }
    }

    fn dequeue(&mut self) -> Option<Self::Elem> {
        if self.as_right_stack().is_empty() {
            while let Some(elem) = self.as_left_stack().pop() {
                self.as_right_stack().push(elem);
            }
        }
        if self.as_right_stack().is_empty() {
            None
        } else {
            self.as_right_stack().pop()
        }
    }
}

/// 共享栈中的左栈.
pub struct LeftStack<'a, T> {
    shared: &'a mut SharedStack<T>,
}

impl<'a, T> Stack for LeftStack<'a, T> {
    type Elem = T;

    fn len(&self) -> usize {
        self.shared.left_len()
    }

    fn cap(&self) -> Option<usize> {
        Some(self.shared.left_cap())
    }

    fn push(&mut self, elem: Self::Elem) -> Option<Self::Elem> {
        if self.is_full() {
            Some(elem)
        } else {
            self.shared.slice[self.shared.ltop] = Some(elem);
            self.shared.ltop += 1;
            None
        }
    }

    fn pop(&mut self) -> Option<Self::Elem> {
        if self.is_empty() {
            None
        } else {
            self.shared.ltop -= 1;
            self.shared.slice[self.shared.ltop].take()
        }
    }

    fn top(&self) -> Option<&Self::Elem> {
        if self.is_empty() {
            None
        } else {
            self.shared.slice[self.shared.ltop - 1].as_ref()
        }
    }
}

/// 共享栈中的右栈.
pub struct RightStack<'a, T> {
    shared: &'a mut SharedStack<T>,
}

impl<'a, T> Stack for RightStack<'a, T> {
    type Elem = T;

    fn len(&self) -> usize {
        self.shared.right_len()
    }

    fn cap(&self) -> Option<usize> {
        Some(self.shared.right_cap())
    }

    fn push(&mut self, elem: Self::Elem) -> Option<Self::Elem> {
        if self.is_full() {
            Some(elem)
        } else {
            self.shared.rtop -= 1;
            self.shared.slice[self.shared.rtop] = Some(elem);
            None
        }
    }

    fn pop(&mut self) -> Option<Self::Elem> {
        if self.is_empty() {
            None
        } else {
            let poped = self.shared.slice[self.shared.rtop].take();
            self.shared.rtop += 1;
            poped
        }
    }

    fn top(&self) -> Option<&Self::Elem> {
        if self.is_empty() {
            None
        } else {
            self.shared.slice[self.shared.rtop].as_ref()
        }
    }
}

/// (复制)切片栈.
pub struct SliceStack<'a, T> {
    top: usize,
    slice: &'a mut [T],
}

impl<'a, T: Clone> From<&'a mut [T]> for SliceStack<'a, T> {
    fn from(slice: &'a mut [T]) -> Self {
        Self { top: 0, slice }
    }
}

/// (默认)切片栈.
pub struct DefaultSliceStack<'a, T>(SliceStack<'a, T>);

impl<'a, T: Clone> Stack for SliceStack<'a, T> {
    type Elem = T;

    fn cap(&self) -> Option<usize> {
        Some(self.slice.len())
    }

    fn len(&self) -> usize {
        self.top
    }

    fn push(&mut self, elem: Self::Elem) -> Option<Self::Elem> {
        if self.is_full() {
            Some(elem)
        } else {
            self.slice[self.top] = elem;
            self.top += 1;
            None
        }
    }

    fn pop(&mut self) -> Option<Self::Elem> {
        if self.is_empty() {
            None
        } else {
            self.top -= 1;
            Some(self.slice[self.top].clone())
        }
    }

    fn top(&self) -> Option<&Self::Elem> {
        if self.is_empty() {
            None
        } else {
            self.slice.get(self.top - 1)
        }
    }
}

impl<'a, T: Default> From<&'a mut [T]> for DefaultSliceStack<'a, T> {
    fn from(slice: &'a mut [T]) -> Self {
        Self(SliceStack { top: 0, slice })
    }
}

impl<'a, T: Default> Stack for DefaultSliceStack<'a, T> {
    type Elem = T;

    fn cap(&self) -> Option<usize> {
        Some(self.0.slice.len())
    }

    fn len(&self) -> usize {
        self.0.top
    }

    fn push(&mut self, elem: Self::Elem) -> Option<Self::Elem> {
        if self.is_full() {
            Some(elem)
        } else {
            self.0.slice[self.0.top] = elem;
            self.0.top += 1;
            None
        }
    }

    fn pop(&mut self) -> Option<Self::Elem> {
        if self.is_empty() {
            None
        } else {
            self.0.top -= 1;
            let mut poped = T::default();
            std::mem::swap(&mut poped, &mut self.0.slice[self.0.top]);
            Some(poped)
        }
    }

    fn top(&self) -> Option<&Self::Elem> {
        if self.is_empty() {
            None
        } else {
            self.0.slice.get(self.0.top - 1)
        }
    }
}

/// 双栈.
pub struct DoubleStack<S>(S, S);

impl<S: Stack + Default> Default for DoubleStack<S> {
    fn default() -> Self {
        Self(S::default(), S::default())
    }
}

fn full_or_cap<S: Stack>(s: &S, maybe_cap: Option<usize>) -> bool {
    let cap = match (s.cap(), maybe_cap) {
        (Some(cap0), Some(cap1)) => cap0.min(cap1),
        (Some(cap0), None) => cap0,
        (None, Some(cap1)) => cap1,
        _ => return false,
    };
    s.len() == cap
}

/// 使用双栈结构实现队列.
// 习题 3.2.3
impl<S: Stack> Queue for DoubleStack<S> {
    type Elem = S::Elem;

    fn is_empty(&self) -> bool {
        self.0.is_empty() && self.1.is_empty()
    }

    fn is_full(&self) -> bool {
        if self.1.is_empty() {
            self.1.is_full() || self.0.cap().map_or(false, |cap| cap == 0)
        } else {
            full_or_cap(&self.0, self.1.cap())
        }
    }

    fn enqueue(&mut self, elem: Self::Elem) -> Option<Self::Elem> {
        if self.is_full() {
            Some(elem)
        } else {
            if full_or_cap(&self.0, self.1.cap()) && self.1.is_empty() {
                while let Some(elem) = self.0.pop() {
                    self.1.push(elem);
                }
            }
            self.0.push(elem)
        }
    }

    fn dequeue(&mut self) -> Option<Self::Elem> {
        if self.1.is_empty() {
            while let Some(elem) = self.0.pop() {
                self.1.push(elem);
            }
        }
        if self.1.is_empty() {
            None
        } else {
            self.1.pop()
        }
    }
}

impl<S: Stack> StackExt for S {}

/// 栈扩展特质.
/// 实现了一些栈算法.
pub trait StackExt: Stack {
    /// 判断是否为合法出栈顺序.
    /// # Correctness
    /// `seq`必须是`[0, seq.len())`的一个排列.
    fn is_valid_pop_sequence(seq: &[usize]) -> bool
    where
        Self: Stack<Elem = usize> + Default,
    {
        let mut stack = Self::default();
        let mut seqs = seq.iter().peekable();
        for idx in 0..=seq.len() {
            while seqs.peek().is_some() && stack.top().as_ref() == seqs.peek() {
                seqs.next();
                stack.pop();
            }
            if idx != seq.len() {
                stack.push(idx);
            }
        }
        seqs.peek().is_none()
    }

    /// 判断是否为输入序列`[base, base + seq.len())`的合法的出栈顺序(利用递归性质).
    fn is_valid_pop_sequence_recurrence(base: usize, seq: &[usize]) -> bool
    where
        Self: Stack<Elem = usize>,
    {
        if seq.is_empty() {
            return true;
        }
        let inputs: Vec<usize> = (base..(base + seq.len())).collect();
        let last = seq[seq.len() - 1];
        if let Some(last) = last.checked_sub(base) {
            if Some(base + last) != inputs.get(last).ok().copied() {
                return false;
            }
            let lhs = &seq[0..last];
            let rhs = &seq[last..seq.len() - 1];
            Self::is_valid_pop_sequence_recurrence(base, lhs)
                && Self::is_valid_pop_sequence_recurrence(last + base + 1, rhs)
        } else {
            false
        }
    }

    /// 判断一个单链表是否为中心对称的.
    /// # Examples
    /// ```
    /// use my_algo::ch2::linked_list::shll::LinkedList;
    /// use my_algo::ch3::StackExt;
    ///
    /// let list = LinkedList::from(vec![1, 2, 1, 2, 1]);
    /// assert!(LinkedList::is_centrosymmetric(&list));
    ///
    /// let list = LinkedList::from(vec![1, 2, 2, 1]);
    /// assert!(LinkedList::is_centrosymmetric(&list));
    ///
    /// let list = LinkedList::from(vec![1, 2, 3, 1]);
    /// assert!(!LinkedList::is_centrosymmetric(&list));
    ///
    /// ```
    // 习题 3.1.4
    fn is_centrosymmetric<'a, 'b: 'a, T: 'a + PartialEq, L: SinglyLinkedList<T>>(
        list: &'b L,
    ) -> bool
    where
        Self: Stack<Elem = &'a T> + Default,
    {
        let center = if list.len() % 2 == 0 {
            None
        } else {
            Some(list.len() / 2)
        };
        let mid = list.len() / 2;
        let mut stack = Self::default();
        let mut cursor = list.cursor_front();
        while let Some(idx) = cursor.index() {
            if Some(idx) != center {
                if idx < mid {
                    let elem = cursor.into_ref().unwrap();
                    stack.push(elem);
                } else {
                    let elem = stack.pop().unwrap();
                    if elem != cursor.peek().unwrap() {
                        return false;
                    }
                }
            }
            cursor.move_next();
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use permutation_iterator::Permutor;
    use proptest::prelude::*;

    #[test]
    fn test_shared_stack() {
        let data = vec![1, 2, 3, 4, 5];
        let mut stack = SharedStack::new(5);
        stack.as_left_stack().push(data[0]);
        stack.as_left_stack().push(data[1]);
        for elem in data.iter().skip(2) {
            assert_eq!(stack.as_right_stack().push(*elem), None)
        }
        assert_eq!(stack.slice, [Some(1), Some(2), Some(5), Some(4), Some(3)]);

        assert_eq!(stack.as_right_stack().push(2), Some(2));
        assert_eq!(stack.as_left_stack().pop(), Some(2));
        assert_eq!(stack.as_right_stack().push(2), None);
        assert_eq!(stack.as_left_stack().push(2), Some(2));
        assert_eq!(stack.as_right_stack().pop(), Some(2));
        assert_eq!(stack.as_left_stack().push(2), None);
        assert_eq!(stack.slice, [Some(1), Some(2), Some(5), Some(4), Some(3)]);

        assert_eq!(stack.as_left_stack().pop(), Some(2));
        assert_eq!(stack.as_left_stack().pop(), Some(1));
        assert_eq!(stack.as_left_stack().pop(), None);

        assert_eq!(stack.as_right_stack().pop(), Some(5));
        assert_eq!(stack.as_right_stack().pop(), Some(4));
        assert_eq!(stack.as_right_stack().pop(), Some(3));
        assert_eq!(stack.as_right_stack().pop(), None);
    }

    #[test]
    fn test_queue_basic_double_stack() {
        let data = vec![1, 2, 3, 4];
        let mut v1 = vec![0; 2];
        let mut v2 = vec![0; 2];
        let s1 = SliceStack::from(v1.as_mut_slice());
        let s2 = SliceStack::from(v2.as_mut_slice());
        let mut queue = DoubleStack(s1, s2);
        for elem in data.iter() {
            assert_eq!(queue.enqueue(*elem), None);
        }
        for elem in data.iter() {
            assert_eq!(queue.dequeue(), Some(*elem));
        }
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_pop_sequence_recurrence() {
        assert!(!cdll::LinkedList::is_valid_pop_sequence_recurrence(
            0,
            &[2, 0, 1]
        ));
        assert!(cdll::LinkedList::is_valid_pop_sequence_recurrence(
            0,
            &[0, 1, 2]
        ));
        assert!(cdll::LinkedList::is_valid_pop_sequence_recurrence(
            0,
            &[0, 2, 1]
        ));
        assert!(cdll::LinkedList::is_valid_pop_sequence_recurrence(
            0,
            &[1, 0, 2]
        ));
        assert!(cdll::LinkedList::is_valid_pop_sequence_recurrence(
            0,
            &[1, 2, 0]
        ));
        assert!(cdll::LinkedList::is_valid_pop_sequence_recurrence(
            0,
            &[2, 1, 0]
        ));
    }

    #[test]
    fn test_pop_sequence() {
        assert!(!cdll::LinkedList::is_valid_pop_sequence(&[2, 0, 1]));
        assert!(cdll::LinkedList::is_valid_pop_sequence(&[0, 1, 2]));
        assert!(cdll::LinkedList::is_valid_pop_sequence(&[0, 2, 1]));
        assert!(cdll::LinkedList::is_valid_pop_sequence(&[1, 0, 2]));
        assert!(cdll::LinkedList::is_valid_pop_sequence(&[1, 2, 0]));
        assert!(cdll::LinkedList::is_valid_pop_sequence(&[2, 1, 0]));
    }

    proptest! {
        #[test]
        fn test_queue_basic_shared_stack(data: Vec<usize>) {
            let mut queue = SharedStack::new(data.len());
            for elem in data.iter() {
                assert_eq!(queue.enqueue(*elem), None);
            }
            for elem in data.iter() {
                assert_eq!(queue.dequeue(), Some(*elem));
            }
            assert_eq!(queue.dequeue(), None);
        }

        #[test]
        fn test_is_centrosymmetric_rand(data: Vec<usize>) {
            let list = shll::LinkedList::from(data.clone());
            let ans = data
                .iter()
                .zip(data.iter().rev())
                .map(|(a, b)| a == b)
                .all(|cmp| cmp);
            prop_assert_eq!(shll::LinkedList::is_centrosymmetric(&list), ans);
        }

        #[test]
        fn test_is_centrosymmetric(mut data: Vec<usize>, center: Option<usize>) {
            let mut right: Vec<_> = data
                .iter()
                .rev()
                .copied()
                .collect();
            if let Some(elem) = center {
                data.push(elem);
            }
            data.append(&mut right);
            let list = shll::LinkedList::from(data);
            prop_assert!(shll::LinkedList::is_centrosymmetric(&list));
        }

        #[test]
        fn test_pop_sequence_all(n in 0..40) {
            let n = n as u64;
            let mut handles = Vec::new();
            for _ in 0..n {
                handles.push(std::thread::spawn(move || {
                    let permutor: Vec<_> = Permutor::new(n).map(|num| num as usize).collect();
                    prop_assert_eq!(
                        cdll::LinkedList::is_valid_pop_sequence(&permutor),
                        cdll::LinkedList::is_valid_pop_sequence_recurrence(0, &permutor)
                    );
                    Ok(())
                }));
            }
            for handle in handles {
                handle.join().unwrap().unwrap();
            }
        }

        #[test]
        fn test_stack_basic_vec(data: Vec<i64>) {
            let mut stack = Vec::new();
            for elem in data.iter() {
                stack.push(*elem);
            }
            for elem in data.iter().rev() {
                prop_assert_eq!(stack.pop(), Some(*elem));
            }
            prop_assert_eq!(stack.pop(), None);
        }

        #[test]
        fn test_stack_basic_myvec(data: Vec<i64>) {
            let mut stack = MyVec::new();
            for elem in data.iter() {
                stack.push(*elem);
            }
            for elem in data.iter().rev() {
                prop_assert_eq!(stack.pop(), Some(*elem));
            }
            prop_assert_eq!(stack.pop(), None);
        }

        #[test]
        fn test_stack_basic_shll(data: Vec<i64>) {
            let mut stack = shll::LinkedList::default();
            for elem in data.iter() {
                stack.push(*elem);
            }
            for elem in data.iter().rev() {
                prop_assert_eq!(stack.pop(), Some(*elem));
            }
            prop_assert_eq!(stack.pop(), None);
        }

        #[test]
        fn test_stack_basic_cdll(data: Vec<i64>) {
            let mut stack = cdll::LinkedList::default();
            for elem in data.iter() {
                stack.push(*elem);
            }
            for elem in data.iter().rev() {
                prop_assert_eq!(stack.pop(), Some(*elem));
            }
            prop_assert_eq!(stack.pop(), None);
        }

        #[test]
        fn test_stack_basic_slice_stack(data: Vec<i64>) {
            let mut v = vec![0; data.len()];
            let mut stack = SliceStack::from(v.as_mut_slice());
            for elem in data.iter() {
                stack.push(*elem);
            }
            for elem in data.iter().rev() {
                prop_assert_eq!(stack.pop(), Some(*elem));
            }
            prop_assert_eq!(stack.pop(), None);
        }

        #[test]
        fn test_stack_basic_default_slice_stack(data: Vec<i64>) {
            let mut v = vec![0; data.len()];
            let mut stack = DefaultSliceStack::from(v.as_mut_slice());
            for elem in data.iter() {
                stack.push(*elem);
            }
            for elem in data.iter().rev() {
                prop_assert_eq!(stack.pop(), Some(*elem));
            }
            prop_assert_eq!(stack.pop(), None);
        }

        #[test]
        fn test_stack_basic_shared_stack_left(data: Vec<i64>) {
            let mut stack = SharedStack::new(data.len());
            for elem in data.iter() {
                stack.as_left_stack().push(*elem);
            }
            for elem in data.iter().rev() {
                prop_assert_eq!(stack.as_left_stack().pop(), Some(*elem));
            }
            prop_assert_eq!(stack.as_left_stack().pop(), None);
        }

        #[test]
        fn test_stack_basic_shared_stack_right(data: Vec<i64>) {
            let mut stack = SharedStack::new(data.len());
            for elem in data.iter() {
                stack.as_right_stack().push(*elem);
            }
            for elem in data.iter().rev() {
                prop_assert_eq!(stack.as_right_stack().pop(), Some(*elem));
            }
            prop_assert_eq!(stack.as_right_stack().pop(), None);
        }

        #[test]
        fn test_stack_basic_shared_stack(data: Vec<i64>) {
            let mut stack = SharedStack::new(data.len());
            for elem in data.iter() {
                stack.push(*elem);
            }
            for elem in data.iter().rev() {
                prop_assert_eq!(stack.pop(), Some(*elem));
            }
            prop_assert_eq!(stack.pop(), None);
        }
    }
}
