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

pub struct CursorMut<'a, T: 'a> {
    index: usize,
    current: Option<&'a mut Node<T>>,
    list: &'a mut LinkedList<T>,
}

impl<'a, T: 'a> CursorMut<'a, T> {
    pub fn new(list: &'a mut LinkedList<T>) -> Self {
        Self {
            index: 0,
            current: list.head.map(|node| unsafe { &mut *node.as_ptr() }),
            list,
        }
    }

    pub fn move_prev(&mut self) {
        self.current = self.current.take().map(|node| unsafe {
            self.index = self
                .index
                .checked_sub(1)
                .unwrap_or_else(|| self.list.len() - 1);
            node.prev.as_mut()
        })
    }

    pub fn move_next(&mut self) {
        self.current = self.current.take().map(|node| unsafe {
            self.index += 1;
            self.index %= self.list.len();
            node.next.as_mut()
        })
    }

    pub fn as_cursor(&self) -> Cursor<T> {
        Cursor {
            index: self.index,
            current: self.current.as_deref(),
            list: self.list,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.current.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.current.as_mut().map(|node| &mut node.elem)
    }

    pub fn is_front(&self) -> bool {
        self.as_cursor().is_front()
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn remove_current(&mut self) -> Option<T> {
        self.remove_current_node().map(|node| node.into_elem())
    }

    fn remove_current_node(&mut self) -> Option<Box<Node<T>>> {
        self.current.take().map(|current| unsafe {
            let prev = current.prev;
            let next = current.next;
            (*prev.as_ptr()).next = next;
            (*next.as_ptr()).prev = prev;
            if next.as_ptr() == current {
                self.list.head = None;
                self.current = None;
            } else if self.list.head.unwrap().as_ptr() == current {
                self.list.head = Some(next);
                self.current = Some(&mut *next.as_ptr());
            } else {
                self.current = Some(&mut *next.as_ptr());
            }
            self.list.len -= 1;
            if !self.list.is_empty() {
                self.index %= self.list.len();
            }
            Box::from_raw(current)
        })
    }
}
