use super::*;

impl<T, L> SinglyLinkedListExt<T> for L where L: SinglyLinkedList<T> {}

pub trait SinglyLinkedListExt<T>: SinglyLinkedList<T> {
    /// 就地逆置.
    // 习题 2.3.5
    fn reverse(&mut self) {
        // a -> b -> c
        // a, b -> c
        // b -> a, c
        // c -> b -> a
        if !self.is_empty() {
            let mut left = Self::default();
            while let Some(elem) = self.pop_front() {
                left.push_front(elem);
            }
            *self = left;
        }
    }
}
