use super::*;

pub struct Iter<'a, T: 'a> {
    current: Option<Link<T>>,
    list: &'a CircularDoublyLinkedList<T>,
}

impl<'a, T: 'a> Iter<'a, T> {
    pub(super) fn new(list: &'a CircularDoublyLinkedList<T>) -> Self {
        Self {
            current: list.head,
            list,
        }
    }
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| unsafe {
            self.current = if node.as_ref().next != self.list.head.unwrap() {
                Some(node.as_ref().next)
            } else {
                None
            };
            &(*node.as_ptr()).elem
        })
    }
}

pub struct IterMut<'a, T: 'a> {
    current: Option<Link<T>>,
    list: &'a mut CircularDoublyLinkedList<T>,
}

impl<'a, T: 'a> IterMut<'a, T> {
    pub(super) fn new(list: &'a mut CircularDoublyLinkedList<T>) -> Self {
        Self {
            current: list.head,
            list,
        }
    }
}

impl<'a, T: 'a> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| unsafe {
            self.current = if node.as_ref().next != self.list.head.unwrap() {
                Some(node.as_ref().next)
            } else {
                None
            };
            &mut (*node.as_ptr()).elem
        })
    }
}
