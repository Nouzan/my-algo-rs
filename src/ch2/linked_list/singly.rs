use super::*;

impl<T, L> SinglyLinkedListExt<T> for L where L: SinglyLinkedList<T> {}

pub trait SinglyLinkedListExt<T>: SinglyLinkedList<T> {}
