use super::*;

impl<'a, T, L> SinglyLinkedListExt<'a, T> for L where L: SinglyLinkedList<'a, T> {}

pub trait SinglyLinkedListExt<'a, T>: SinglyLinkedList<'a, T> {}
