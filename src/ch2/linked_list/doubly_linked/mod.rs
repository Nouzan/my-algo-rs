pub mod algos;
pub mod cursor;
pub mod iter;

pub use cursor::*;
pub use iter::*;
use std::cmp::PartialEq;
use std::marker::PhantomData;
use std::ptr::NonNull;

type Link<T> = NonNull<Node<T>>;

struct Node<T> {
    prev: Link<T>,
    next: Link<T>,
    elem: T,
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Self {
        Self {
            prev: NonNull::dangling(),
            next: NonNull::dangling(),
            elem,
        }
    }

    pub fn into_elem(self) -> T {
        self.elem
    }
}

/// 循环双链表.
///
/// 尾结点的后继是首结点, 首结点的前驱是尾结点.
/// 每一个结点因此与其前驱`prev`和后继`next`分别共享了一个指针`Link`;
/// 同时, 首结点还共享了一个指针`head`给链表主体`LinkedList`.
/// 我们必须恰当地管理这些共享指针, 以使得所有公共接口符合`Rust`的引用规则(公共接口仅能返回共享的只读引用和互斥的可变引用).
/// 关键在于以下不变式:
/// 1. 链表内所有关联指针(若有)都是合法指针(前驱、后继以及头指针).
/// 2. 在链表所有权范围内, 任何结点除了首结点外, 仅恰好共享了2个指针, 分别共享给了它的前驱和后继; 首结点还把指针共享给了头指针.
/// 3. 链表内所有结点指针都来源于`Box::leak`.
/// 4. 链表保持循环双链表的性质.
#[derive(Debug)]
pub struct LinkedList<T> {
    len: usize,
    head: Option<Link<T>>,

    /// 告诉dropck, `LinkedList<T>`的确拥有一些`Box<Node<T>>`,
    /// 且在它drop的时候, 我们会将它们也一并drop了.
    marker: PhantomData<Box<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<Vec<T>> for LinkedList<T> {
    fn from(mut data: Vec<T>) -> Self {
        let mut list = Self::new();
        while let Some(elem) = data.pop() {
            list.push_front(elem);
        }
        list
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        let mut q = other.iter();
        for elem in self.iter() {
            if q.next() != Some(elem) {
                return false;
            }
        }
        q.next().is_none()
    }
}

impl<T: PartialEq> PartialEq<Vec<T>> for LinkedList<T> {
    fn eq(&self, other: &Vec<T>) -> bool {
        let mut q = other.iter();
        for elem in self.iter() {
            if q.next() != Some(elem) {
                return false;
            }
        }
        q.next().is_none()
    }
}

impl<T: PartialEq> PartialEq<LinkedList<T>> for Vec<T> {
    fn eq(&self, other: &LinkedList<T>) -> bool {
        other.eq(self)
    }
}

impl<T> LinkedList<T> {
    /// 创建一个空的链表.
    pub fn new() -> Self {
        Self {
            len: 0,
            head: None,
            marker: PhantomData::default(),
        }
    }

    /// 判断表是否为空.
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// 获取表的长度.
    pub fn len(&self) -> usize {
        self.len
    }

    /// 把新值作为新的首结点插入.
    pub fn push_front(&mut self, elem: T) {
        self.push_front_node(Box::new(Node::new(elem)))
    }

    /// 把新值作为新的尾结点插入.
    pub fn push_back(&mut self, elem: T) {
        let mut cursor = self.cursor_back_mut();
        cursor.insert_after(elem);
    }

    /// 弹出首结点, 首结点的直接后继(若有)将成为新的首结点.
    pub fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node().map(|node| node.into_elem())
    }

    /// 弹出尾结点.
    pub fn pop_back(&mut self) -> Option<T> {
        let mut cursor = self.cursor_back_mut();
        cursor.remove_current()
    }

    /// 冻结链表, 创建指向首结点(若有)的只读游标.
    pub fn cursor_front(&self) -> Cursor<T> {
        Cursor::new(self)
    }

    /// 创建指向首结点(若有)的可变游标.
    pub fn cursor_front_mut(&mut self) -> CursorMut<T> {
        CursorMut::new(self)
    }

    /// 冻结链表, 创建指向尾结点(若有)的只读游标.
    pub fn cursor_back(&self) -> Cursor<T> {
        let mut cursor = self.cursor_front();
        cursor.move_prev();
        cursor
    }

    /// 创建指向尾结点(若有)的可变游标.
    pub fn cursor_back_mut(&mut self) -> CursorMut<T> {
        let mut cursor = self.cursor_front_mut();
        cursor.move_prev();
        cursor
    }

    /// 连接两个链表.
    /// `other`将会变为空表.
    pub fn append(&mut self, other: &mut Self) {
        while let Some(elem) = other.pop_front() {
            self.push_back(elem)
        }
    }

    /// 获得一个从首结点到尾结点的只读迭代器.
    pub fn iter(&self) -> Iter<T> {
        Iter::new(self)
    }

    /// 获得一个从首结点到尾结点的可变迭代器.
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut::new(self)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front_node().is_some() {}
    }
}

impl<T> LinkedList<T> {
    /// 在链表前部插入一个新的结点.
    /// 1. 插入后, 被插入的结点将成为新的首结点, 因此共享其指针给`head`;
    /// 2. 新的首结点的后继是原来的首结点(若有, 若没有则为自己), 前驱是尾结点(若有, 若没有则为自己);
    /// 3. 原来的首结点的前驱从尾结点改为新的首结点, 后继不变;
    /// 4. 原来的尾结点的后继从原来的首结点改为新的首结点, 前驱不变.
    fn push_front_node(&mut self, node: Box<Node<T>>) {
        // # Safety
        // 该过程保持了不变式: 链表内所有关联指针(若有)都是合法指针(前驱、后继以及头指针).
        unsafe {
            // 泄漏`node`, 以避免它被drop.
            let node: Link<T> = Box::leak(node).into();
            match self.head {
                None => {
                    // 原来的表为空, 因此实现(2)中的“若没有”部分.
                    // Safety: 这里解引用指针是安全的, 是因为`node`是刚刚通过`Box::leak`得到的指针.
                    // 同时这里也保持了不变式.
                    (*node.as_ptr()).next = node;
                    (*node.as_ptr()).prev = node;
                }
                Some(head) => {
                    // 这里实现了(2).
                    (*node.as_ptr()).next = head;

                    // 原首结点的前驱是尾结点.
                    // Safety: 根据不变式, 对`head`解引用指针是安全的.
                    (*node.as_ptr()).prev = (*head.as_ptr()).prev;

                    // 这里实现了(4).
                    (*(*head.as_ptr()).prev.as_ptr()).next = node;

                    // 这里实现了(3).
                    (*head.as_ptr()).prev = node;

                    // 经过上述过程, 我们得到:
                    // - `node`的前驱和后继分别是`head`和`head->prev`, 根据不变式都是合法的.
                    // - `head`的后继若不是自己则不变, 前驱变为`node`; 若后继是自己, 则后继也为`node`, 都是合法的.
                    // - `head->prev`的前驱若不是自己则不变, 后继变为`node`; 若前驱是自己, 则前驱变为`node`, 都是合法的.
                    // `node`的指针仅共享给了`head`和`head->prev`(它们可以是同一个结点), 且所有其它指针均未丢失.
                    // 操作不涉及其余结点, 因此上述过程保持不变式.
                }
            }
            // 共享其指针给`head`(1). 这保持了头指针的不变式.
            self.head = Some(node);
            self.len += 1;
        }
    }

    /// 弹出链表的首结点. 若表空则返回`None`.
    /// 1. 弹出后, 首结点的后继(若有)成为新的首结点;
    /// 2. 首结点的后继的前驱变为尾结点(首结点的前驱), 尾结点的后继变为首结点的后继.
    fn pop_front_node(&mut self) -> Option<Box<Node<T>>> {
        // # Safety
        // 该过程保持了不变式: 链表内所有关联指针(若有)都是合法指针(前驱、后继以及头指针).
        self.head.map(|head| unsafe {
            let next = (*head.as_ptr()).next;
            if head == next {
                // 首结点是链表的唯一结点.
                self.head = None;
            } else {
                // Safety: 根据不变式1, 所有解引用都是安全的.
                // 这里也保持了不变式1、2.
                (*next.as_ptr()).prev = (*head.as_ptr()).prev;
                (*(*head.as_ptr()).prev.as_ptr()).next = next;
                self.head = Some(next);
            }
            self.len -= 1;
            // Safety: 根据不变式2、3, 这里是安全的.
            Box::from_raw(head.as_ptr())
        })
    }
}

#[cfg(test)]
mod test;
