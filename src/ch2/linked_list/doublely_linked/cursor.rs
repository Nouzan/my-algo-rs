use super::*;

pub struct Cursor<'a, T: 'a> {
    index: usize,
    current: Option<Link<T>>,
    list: &'a LinkedList<T>,
}

impl<'a, T: 'a> Cursor<'a, T> {
    pub fn new(list: &'a LinkedList<T>) -> Self {
        Self {
            index: 0,
            current: list.head,
            list,
        }
    }

    pub fn move_prev(&mut self) {
        self.current = self.current.take().map(|node| unsafe {
            self.index = self
                .index
                .checked_sub(1)
                .unwrap_or_else(|| self.list.len() - 1);
            node.as_ref().prev
        })
    }

    pub fn move_next(&mut self) {
        self.current = self.current.take().map(|node| unsafe {
            self.index += 1;
            self.index %= self.list.len();
            node.as_ref().next
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.current.map(|node| unsafe { &(*node.as_ptr()).elem })
    }

    pub fn is_front_or_empty(&self) -> bool {
        self.list.head == self.current
    }

    pub fn index(&self) -> Option<usize> {
        if self.current.is_some() {
            Some(self.index)
        } else {
            None
        }
    }
}

pub struct CursorMut<'a, T: 'a> {
    index: usize,
    current: Option<Link<T>>,
    list: &'a mut LinkedList<T>,
}

impl<'a, T: 'a> CursorMut<'a, T> {
    pub fn new(list: &'a mut LinkedList<T>) -> Self {
        Self {
            index: 0,
            current: list.head,
            list,
        }
    }

    pub fn move_prev(&mut self) {
        self.current = self.current.take().map(|node| unsafe {
            self.index = self
                .index
                .checked_sub(1)
                .unwrap_or_else(|| self.list.len() - 1);
            node.as_ref().prev
        })
    }

    pub fn move_next(&mut self) {
        self.current = self.current.take().map(|node| unsafe {
            self.index += 1;
            self.index %= self.list.len();
            node.as_ref().next
        })
    }

    pub fn as_cursor(&self) -> Cursor<T> {
        Cursor {
            index: self.index,
            current: self.current,
            list: self.list,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.current.map(|node| unsafe { &(*node.as_ptr()).elem })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.current
            .map(|node| unsafe { &mut (*node.as_ptr()).elem })
    }

    pub fn is_front_or_empty(&self) -> bool {
        self.as_cursor().is_front_or_empty()
    }

    pub fn index(&self) -> Option<usize> {
        if self.current.is_some() {
            Some(self.index)
        } else {
            None
        }
    }

    pub fn insert_before(&mut self, elem: T) {
        self.insert_before_node(Box::new(Node::new(elem)));
    }

    pub fn insert_after(&mut self, elem: T) {
        self.insert_after_node(Box::new(Node::new(elem)));
    }

    pub fn remove_current(&mut self) -> Option<T> {
        self.remove_current_node().map(|node| node.into_elem())
    }
}

impl<'a, T: 'a> CursorMut<'a, T> {
    fn insert_after_node(&mut self, node: Box<Node<T>>) {
        if self.list.is_empty() {
            self.list.push_front_node(node);
            self.current = self.list.head;
        } else {
            let node: Link<T> = Box::leak(node).into();
            unsafe {
                let current = self.current.unwrap();
                let next = current.as_ref().next;
                (*node.as_ptr()).next = next;
                (*node.as_ptr()).prev = current;
                (*current.as_ptr()).next = node;
                (*next.as_ptr()).prev = node;
            }
            self.list.len += 1;
        }
    }

    fn insert_before_node(&mut self, node: Box<Node<T>>) {
        if self.is_front_or_empty() {
            self.list.push_front_node(node);
            if self.current.is_none() {
                self.current = self.list.head;
            }
        } else {
            let node: Link<T> = Box::leak(node).into();
            unsafe {
                let current = self.current.unwrap();
                let prev = current.as_ref().prev;
                (*node.as_ptr()).next = current;
                (*node.as_ptr()).prev = prev;
                (*prev.as_ptr()).next = node;
                (*current.as_ptr()).prev = node;
            }
            self.index += 1;
            self.list.len += 1;
        }
    }

    fn remove_current_node(&mut self) -> Option<Box<Node<T>>> {
        self.current.take().map(|current| unsafe {
            let prev = current.as_ref().prev;
            let next = current.as_ref().next;
            (*prev.as_ptr()).next = next;
            (*next.as_ptr()).prev = prev;
            if next == current {
                self.list.head = None;
                self.current = None;
            } else if self.list.head.unwrap() == current {
                self.list.head = Some(next);
                self.current = Some(next);
            } else {
                self.current = Some(next);
            }
            self.list.len -= 1;
            if !self.list.is_empty() {
                self.index %= self.list.len();
            }
            Box::from_raw(current.as_ptr())
        })
    }
}
