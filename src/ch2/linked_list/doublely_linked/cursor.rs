use super::*;

pub struct Cursor<'a, T: 'a> {
    index: usize,
    current: Option<&'a Node<T>>,
    list: &'a LinkedList<T>,
}

impl<'a, T: 'a> Cursor<'a, T> {
    pub fn new(list: &'a LinkedList<T>) -> Self {
        Self {
            index: 0,
            current: list.head.map(|node| unsafe { &*node.as_ptr() }),
            list,
        }
    }

    pub fn move_prev(&mut self) {
        self.current = self.current.take().map(|node| unsafe {
            self.index = self
                .index
                .checked_sub(1)
                .unwrap_or_else(|| self.list.len() - 1);
            node.prev.as_ref()
        })
    }

    pub fn move_next(&mut self) {
        self.current = self.current.take().map(|node| unsafe {
            self.index += 1;
            self.index %= self.list.len();
            node.next.as_ref()
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.current.as_ref().map(|node| &node.elem)
    }

    pub fn is_front(&self) -> bool {
        if let Some(node) = self.current.as_ref() {
            let ptr: *const _ = *node;
            self.list.head.unwrap().as_ptr() as *const _ == ptr
        } else {
            false
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }
}
