pub mod utils;

use super::*;
use std::cmp::PartialEq;

type Link<T> = Option<Box<Node<T>>>;

/// 普通结点.
#[derive(Debug)]
pub struct ItemNode<T> {
    elem: T,
    next: Link<T>,
}

/// 单链表结点.
#[derive(Debug)]
pub enum Node<T> {
    Item(ItemNode<T>),
    Head(Link<T>),
}

impl<T> Node<T> {
    /// 创建一个新的普通结点.
    fn new(elem: T) -> Box<Self> {
        Box::new(Node::Item(ItemNode { elem, next: None }))
    }

    /// 转换为一个`ItemNode`的可变引用.
    /// # Panics
    /// 如果对`Head`进行转换则报错.
    fn as_item_node_mut_unchecked(&mut self) -> &mut ItemNode<T> {
        match self {
            Self::Head(_) => panic!("A `Head` cannot be make into an `ItemNode`."),
            Self::Item(node) => node,
        }
    }

    /// 转换为一个`ItemNode`的只读引用.
    /// # Panics
    /// 如果对`Head`进行转换则报错.
    fn as_item_node_unchecked(&self) -> &ItemNode<T> {
        match self {
            Self::Head(_) => panic!("A `Head` cannot be make into an `ItemNode`."),
            Self::Item(node) => node,
        }
    }

    /// 转换为一个`ItemNode`.
    /// # Panics
    /// 如果对`Head`进行转换则报错.
    fn into_item_node_unchecked(self) -> ItemNode<T> {
        match self {
            Self::Head(_) => panic!("A `Head` cannot be make into an `ItemNode`."),
            Self::Item(node) => node,
        }
    }

    /// 分割引用.
    /// # Panics
    /// 如果对`Head`进行引用分割则报错.
    fn split_mut_unchecked(&mut self) -> (Option<&mut Node<T>>, &mut T) {
        match self {
            Self::Head(_) => panic!("Trying to get value from `Head`."),
            Self::Item(node) => (node.next.as_deref_mut(), &mut node.elem),
        }
    }

    /// 返回当前结点后继的只读引用, 若后继为`None`则返回`None`.
    fn next(&self) -> Option<&Node<T>> {
        match self {
            Self::Head(next) => next.as_ref().map(|node| &**node),
            Self::Item(node) => node.next.as_deref(),
        }
    }

    /// 返回当前结点后继的可变引用, 若后继为`None`则返回`None`.
    fn next_mut(&mut self) -> Option<&mut Node<T>> {
        match self {
            Self::Head(next) => next.as_mut().map(|node| &mut **node),
            Self::Item(node) => node.next.as_deref_mut(),
        }
    }

    /// 返回当前结点内容的可变引用.
    /// # Panics
    /// 方法假设当前结点为普通结点(`ItemNode`), 若不是普通结点, 则报错.
    fn elem_mut_unchecked(&mut self) -> &mut T {
        let (_, elem) = self.split_mut_unchecked();
        elem
    }

    /// 返回当前结点内容的只读引用.
    /// # Panics
    /// 方法假设当前结点为普通结点(`ItemNode`), 若不是普通结点, 则报错.
    fn elem_unchecked(&self) -> &T {
        match self {
            Self::Head(_) => panic!("Trying to get value from `Head`."),
            Self::Item(node) => &node.elem,
        }
    }

    /// 将当前结点的后继替换为`next`, 并返回原来的后继.
    fn link(&mut self, next: Link<T>) -> Link<T> {
        match self {
            Self::Head(link) => {
                let old = link.take();
                *link = next;
                old
            }
            Self::Item(node) => {
                let old = node.next.take();
                node.next = next;
                old
            }
        }
    }
}

/// 带头结点的单链表.
/// # Correctness
/// 该单链表的实现必须保证任何时候任何结点的后继都是`Option<ItemNode>`.
pub struct LinkedList<T> {
    head: Box<Node<T>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self {
            head: Box::new(Node::Head(None)),
        }
    }
}

impl<T> From<Vec<T>> for LinkedList<T> {
    fn from(data: Vec<T>) -> Self {
        let mut list = Self::default();
        let mut cursor = list.cursor_front_mut();
        for v in data {
            cursor.insert_after_as_current(v);
        }
        list
    }
}

impl<T> From<LinkedList<T>> for Vec<T> {
    fn from(mut linked_list: LinkedList<T>) -> Self {
        let mut list = vec![];
        let mut cursor = linked_list.cursor_front_mut();
        while cursor.peek().is_some() {
            list.push(cursor.remove_current().unwrap()) // 已经判空, 故可`unwrap`.
        }
        list
    }
}

impl<T: PartialEq> PartialEq<Vec<T>> for LinkedList<T> {
    fn eq(&self, other: &Vec<T>) -> bool {
        for (idx, v) in self.iter().enumerate() {
            if !(idx < other.len() && *v == other[idx]) {
                return false;
            }
        }
        true
    }
}

impl<T: PartialEq> PartialEq<LinkedList<T>> for Vec<T> {
    fn eq(&self, other: &LinkedList<T>) -> bool {
        other.eq(self)
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, "[]")
        } else {
            let mut iter = self.iter();
            write!(f, "[{:?}", iter.next().unwrap())?;
            for elem in iter {
                write!(f, ", {:?}", elem)?;
            }
            write!(f, "]")?;
            Ok(())
        }
    }
}

/// 单链表可变迭代器.
pub struct IterMut<'a, T: 'a> {
    inner: Option<&'a mut ItemNode<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.take().map(|node| {
            let (next, elem) = (
                node.next
                    .as_mut()
                    .map(|node| node.as_item_node_mut_unchecked()),
                &mut node.elem,
            );
            self.inner = next;
            elem
        })
    }
}

/// 单链表只读迭代器.
pub struct Iter<'a, T: 'a> {
    inner: Option<&'a ItemNode<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.take().map(|node| {
            let (next, elem) = (
                node.next.as_ref().map(|node| node.as_item_node_unchecked()),
                &node.elem,
            );
            self.inner = next;
            elem
        })
    }
}

/// 只读单链表游标.
/// 该结构保存着的是“当前”结点的直接前驱(`prev`).
///
/// 对于一个带头结点的单链表的游标, 当前结点有可能是以下三种情况之一:
/// - 任何普通结点: `prev`为普通结点的前驱(可能为头结点)
/// - 尾结点的后继: `None`, 此时`prev`为尾结点.
/// - 尾结点的后继的后继: `None`的后继, 此时`prev`为`None`.
/// # Correctness
/// 必须保证`prev`结点的所有后继都是`Option<ItemNode>`.
#[derive(Debug)]
pub struct Cursor<'a, T: 'a> {
    index: usize,
    prev: Option<&'a Node<T>>, // 始终保存着“当前”结点的前驱, 因此“当前”结点必然为`Option<ItemNode>`.
}

impl<'a, T> Clone for Cursor<'a, T> {
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            prev: self.prev,
        }
    }
}

impl<'a, T> Copy for Cursor<'a, T> {}

impl<'a, T> Cursor<'a, T> {
    /// 转换为内容的只读引用.
    pub fn into_inner(mut self) -> Option<&'a T> {
        if let Some(prev) = self.prev.take() {
            prev.next().map(|node| node.elem_unchecked())
        } else {
            None
        }
    }
}

impl<'a, T: 'a> LinearCursor<T> for Cursor<'a, T> {
    /// 注意: 如果当前结点为`None`或`None`的后继, 则返回`false`.
    fn is_front_or_empty(&self) -> bool {
        matches!(self.prev.as_deref(), Some(Node::Head(_)))
    }

    /// 注意: 如果当前结点为`None`或`None`的后继, 则返回`false`.
    fn is_empty(&self) -> bool {
        matches!(self.prev.as_deref(), Some(Node::Head(None)))
    }

    fn is_ghost(&self) -> bool {
        match self.prev.as_deref() {
            Some(node) => node.next().is_none(),
            None => true,
        }
    }

    /// 当前结点的下标.
    /// 若当前结点为`None`或`None`的后继, 则返回`None`.
    fn index(&self) -> Option<usize> {
        if self.peek().is_some() {
            Some(self.index)
        } else {
            None
        }
    }

    /// 移动游标到当前结点的直接后继.
    /// 逻辑上, 游标可以指向`None`的后继(即`prev`为`None`).
    fn move_next(&mut self) {
        // # Correctness
        // 如果`prev`的所有后继都是`Option<ItemNode>`,
        // 那么`prev`的后继的所有后继自然也是`Option<ItemNode>`.
        // 故依然保持`Correctness`假设.
        //
        // 反复调用该方法最终必然会使得`prev`为`None`,
        // 而当`prev`为`None`时, 该方法是`no-op`.
        if let Some(prev) = self.prev.take() {
            self.index += 1;
            self.prev = prev.next();
        }
    }

    /// 获取当前结点内容的只读引用.
    /// 如果游标的当前结点为链表末尾的`None`或`None`的后继, 则返回`None`.
    fn peek(&self) -> Option<&T> {
        if let Some(prev) = self.prev.as_ref() {
            prev.next().as_ref().map(|node| {
                node.elem_unchecked() // `node`是后继, 因此必然是`ItemNode`.
            })
        } else {
            None
        }
    }
}

/// 可变单链表游标.
/// 该结构保存着的是“当前”结点的直接前驱(`prev`).
///
/// 对于一个带头结点的单链表的游标, 当前结点有可能是以下三种情况之一:
/// - 任何普通结点: `prev`为普通结点的前驱(可能为头结点)
/// - 尾结点的后继: `None`, 此时`prev`为尾结点.
/// - 尾结点的后继的后继: `None`的后继, 此时`prev`为`None`.
/// # Correctness
/// 必须保证`prev`结点的所有后继都是`Option<ItemNode>`.
pub struct CursorMut<'a, T: 'a> {
    index: usize,
    prev: Option<&'a mut Node<T>>, // 始终保存着“当前”结点的前驱, 因此“当前”结点必然为`Option<ItemNode>`.
}

impl<'a, T> CursorMut<'a, T> {
    /// 移除当前结点并返回, 当前结点将会变为原来结点的后继.
    /// - 如果游标的当前结点为尾结点, 那么移除后, 当前结点变为`None`;
    /// - 如果游标的当前结点为`None`或`None`的后继, 则该方法是`no-op`, 返回值为`None`.
    fn remove_current_as_node(&mut self) -> Link<T> {
        // # Correctness
        // 如果`prev`的所有后继都是`Option<ItemNode>`,
        // 那么`prev`的后继的所有后继自然也是`Option<ItemNode>`.
        // 故依然保持`Correctness`假设.
        if let Some(prev) = self.prev.as_mut() {
            let mut current = prev.link(None);
            if let Some(node) = &mut current {
                let next = node.link(None);
                prev.link(next);
            }
            current
        } else {
            None
        }
    }

    /// 在当前结点后面插入一个新的结点作为当前结点的后继, 游标仍然指向当前结点(若在尾结点的后继之后插入, 则游标指向新的尾结点).
    /// 若插入成功则返回`None`, 否则返回`elem`.
    /// - 若当前结点是`None`(即当前结点为尾结点的后继, `prev`为末尾结点或头结点), 则将会在末尾结点之后进行插入, 并返回`None`.
    /// - 若当前结点`None`的后继, 将不会进行插入, 并返回`Some(elem)`.
    /// - 其余情况均能按预期进行插入, 并返回`None`.
    fn insert_after_naive(&mut self, elem: T) -> Option<T> {
        // # Correctness
        // 新插入结点是一个普通结点, 且作为`prev`的后继的后继插入, 因此依然保持`Correctness`的假设.
        if let Some(prev) = self.prev.as_mut() {
            let node = match prev.next_mut() {
                Some(node) => node,
                None => prev,
            };
            let next = node.link(Some(Node::new(elem)));
            node.next_mut().unwrap().link(next);
            None
        } else {
            Some(elem)
        }
    }
}

impl<'a, T: 'a> LinearCursor<T> for CursorMut<'a, T> {
    /// 当前结点的下标.
    /// 若当前结点为`None`或`None`的后继, 则返回`None`.
    fn index(&self) -> Option<usize> {
        if self.as_cursor().peek().is_some() {
            Some(self.index)
        } else {
            None
        }
    }

    /// 移动游标到当前结点的直接后继.
    /// 逻辑上, 游标可以指向`None`的后继(即`prev`为`None`).
    fn move_next(&mut self) {
        // # Correctness
        // 如果`prev`的所有后继都是`Option<ItemNode>`,
        // 那么`prev`的后继的所有后继自然也是`Option<ItemNode>`.
        // 故依然保持`Correctness`假设.
        //
        // 反复调用该方法最终必然会使得`prev`为`None`,
        // 而当`prev`为`None`时, 该方法是`no-op`.
        if let Some(prev) = self.prev.take() {
            self.index += 1;
            self.prev = prev.next_mut();
        }
    }

    fn peek(&self) -> Option<&T> {
        if let Some(prev) = self.prev.as_ref() {
            prev.next().map(|node| node.elem_unchecked())
        } else {
            None
        }
    }

    fn is_front_or_empty(&self) -> bool {
        self.as_cursor().is_front_or_empty()
    }

    fn is_empty(&self) -> bool {
        self.as_cursor().is_empty()
    }

    fn is_ghost(&self) -> bool {
        self.as_cursor().is_ghost()
    }
}

impl<'a, 'b, T: 'a + 'b> LinearCursorMut<'b, T> for CursorMut<'a, T> {
    type Cursor = Cursor<'b, T>;

    fn as_cursor(&'b self) -> Self::Cursor {
        Cursor {
            index: self.index,
            prev: self.prev.as_deref(),
        }
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        if let Some(prev) = self.prev.as_mut() {
            prev.next_mut().map(|node| {
                node.elem_mut_unchecked() // `node`是后继, 因此必然是`ItemNode`.
            })
        } else {
            None
        }
    }

    /// 移除当前结点并返回它的内容, 当前结点将会变为原来结点的后继.
    /// - 如果游标的当前结点为尾结点, 那么移除后, 当前结点变为`None`;
    /// - 如果游标的当前结点为`None`或`None`的后继, 则该方法是`no-op`, 返回值为`None`.
    fn remove_current(&mut self) -> Option<T> {
        // 当前结点必然是后继, 后继必然是`Option<ItemNode>`.
        self.remove_current_as_node()
            .map(|node| node.into_item_node_unchecked().elem)
    }

    /// 在当前结点前面插入一个新的结点作为当前结点的新前驱, 游标的当前结点变为该新前驱.
    /// 若插入成功则返回`None`, 否则返回`elem`.
    /// - 若当前结点是`None`(`prev`为末尾结点或头结点), 则将会在末尾结点之后进行插入, 并返回`None`.
    /// - 若当前结点是`None`的后继(`prev`为`None`), 则不会进行插入(且游标不会发生移动), 同时返回`Some(elem)`.
    /// - 其它情况均能按照预期进行插入, 并返回`None`.
    fn insert_before_as_current(&mut self, elem: T) -> Option<T> {
        // # Correctness
        // 新插入结点是一个普通结点, 且作为`prev`的后继进行插入, 因此依然保持`Correctness`的假设.
        if let Some(prev) = self.prev.as_mut() {
            let next = prev.link(Some(Node::new(elem)));
            prev.next_mut().unwrap().link(next); // `next_mut`返回的是刚刚插入的结点的引用, 故必然不为`None`.
            None
        } else {
            Some(elem)
        }
    }

    fn insert_before(&mut self, elem: T) -> Option<T> {
        if self.is_ghost() {
            self.insert_before_as_current(elem)
        } else {
            let res = self.insert_before_as_current(elem);
            self.move_next();
            res
        }
    }

    fn insert_after(&mut self, elem: T) -> Option<T> {
        self.insert_after_naive(elem)
    }

    fn insert_after_as_current(&mut self, elem: T) -> Option<T> {
        if self.is_ghost() {
            self.insert_after_naive(elem)
        } else {
            self.insert_after_naive(elem);
            self.move_next();
            None
        }
    }
}

impl<T> LinkedList<T> {
    /// 返回一个从首结点开始的只读迭代器.
    pub fn iter(&self) -> Iter<T> {
        Iter {
            inner: self.head.next().map(|node| node.as_item_node_unchecked()),
        }
    }

    /// 返回一个从首结点开始的可变迭代器.
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            inner: self
                .head
                .next_mut()
                .map(|node| node.as_item_node_mut_unchecked()),
        }
    }

    /// 就地逆置.
    // 习题 2.3.5
    pub fn reverse(&mut self) {
        // a -> b -> c
        // a, b -> c
        // b -> a, c
        // c -> b -> a
        if !self.is_empty() {
            // a -> b -> c => a, b -> c
            let mut right = Self::default();
            let rest = self.head.next_mut().unwrap().link(None); // 已经判过空, 因此可以`unwrap`.
            right.head.link(rest);
            let mut cursor_left = self.cursor_front_mut();
            let mut cursor_right = right.cursor_front_mut();
            while cursor_right.peek().is_some() {
                let elem = cursor_right.remove_current().unwrap(); // 已经判过空, 因此可以`unwrap`.
                cursor_left.insert_before_as_current(elem);
            }
        }
    }

    /// 连接两个链表.
    pub fn append(&mut self, mut rhs: Self) {
        let mut cursor = self.cursor_front_mut();
        while cursor.peek().is_some() {
            cursor.move_next()
        }
        cursor.prev.unwrap().link(rhs.head.link(None));
    }

    /// 获取倒数第n个元素.
    /// 若表不够长, 则返回`None`.
    /// # Examples
    /// ```
    /// use my_algo::ch2::linked_list::shll::LinkedList;
    ///
    /// let list = LinkedList::from(vec![5, 4, 3, 2, 1]);
    /// assert_eq!(list.last(1), Some(&1));
    /// assert_eq!(list.last(0), None);
    /// assert_eq!(list.last(6), None);
    /// assert_eq!(list.last(5), Some(&5));
    /// ```
    // 习题 2.3.21
    pub fn last(&self, n: usize) -> Option<&T> {
        if n == 0 {
            return None;
        }
        let mut lcur = self.cursor_front();
        let mut rcur = self.cursor_front();
        let mut flag = false;
        while rcur.peek().is_some() {
            if flag {
                lcur.move_next();
            }
            if rcur.index().unwrap() - lcur.index().unwrap() >= n - 1 {
                flag = true;
            }
            rcur.move_next();
        }
        if flag {
            lcur.into_inner()
        } else {
            None
        }
    }
}

impl<'a, T: 'a> SinglyLinkedList<'a, T> for LinkedList<T> {
    type Cursor = Cursor<'a, T>;
    type CursorMut = CursorMut<'a, T>;

    fn is_empty(&self) -> bool {
        self.head.next().is_none()
    }

    /// 返回一个当前结点为首结点的可变游标.
    fn cursor_front_mut(&mut self) -> CursorMut<T> {
        CursorMut {
            index: 0,
            prev: Some(self.head.as_mut()),
        }
    }

    /// 返回一个当前结点为首结点的只读游标.
    fn cursor_front(&self) -> Cursor<T> {
        Cursor {
            index: 0,
            prev: Some(self.head.as_ref()),
        }
    }

    /// 在链表最前面插入新元素作为新的头结点.
    fn push_front(&mut self, elem: T) {
        self.cursor_front_mut().insert_before(elem);
    }

    /// 弹出链表最前面的元素.
    /// 若表空则返回`None`.
    fn pop_front(&mut self) -> Option<T> {
        self.cursor_front_mut().remove_current()
    }
}

impl<T: PartialEq> LinkedList<T> {
    /// 删除所有值等于`x`的元素.
    pub fn delete_all(&mut self, x: &T) {
        let mut cursor = self.cursor_front_mut();
        while let Some(current) = cursor.peek() {
            if *current == *x {
                cursor.remove_current();
            }
            cursor.move_next();
        }
    }

    /// 去除连续重复的元素.
    ///
    /// 若单链表是有序的, 这将去除所有重复元素.
    /// # Examples
    /// ```
    /// use my_algo::ch2::linked_list::shll::LinkedList;
    ///
    /// let mut list = LinkedList::from(vec![1, 2, 2, 3, 2]);
    /// list.dedup();
    /// assert_eq!(list, vec![1, 2, 3, 2]);
    /// ```
    // 习题 2.3.12
    pub fn dedup(&mut self) {
        let mut cursor = self.cursor_front_mut();
        let mut pionner = cursor.as_cursor();
        pionner.move_next();
        while let Some(elem) = pionner.peek() {
            if *elem == *cursor.as_cursor().peek().unwrap() {
                cursor.remove_current();
            } else {
                cursor.move_next();
            }
            pionner = cursor.as_cursor();
            pionner.move_next();
        }
    }
}

use std::cmp::PartialOrd;

impl<T: PartialOrd> LinkedList<T> {
    /// 删除第一次出现的最小值结点.
    /// 若表空, 则返回`None`.
    // 习题 2.3.4
    pub fn pop_min(&mut self) -> Option<T> {
        if !self.is_empty() {
            let mut cursor = self.cursor_front_mut(); // 指向已知最小值的游标, 由于表非空, 开始时指向首结点.
            let mut pionner = cursor.as_cursor(); // 先锋游标.
            pionner.move_next(); // 先锋前进一步.
            while let Some(elem) = pionner.peek() {
                if *elem < *cursor.as_cursor().peek().unwrap() {
                    let idx = pionner.index().unwrap(); // 已经经过判空, 这里可以直接`unwrap`.
                    while cursor.index().unwrap() != idx {
                        // 追上先锋.
                        cursor.move_next();
                    }
                    pionner = cursor.as_cursor();
                }
                pionner.move_next();
            }
            cursor.remove_current()
        } else {
            None
        }
    }

    /// 快速排序中的helper.
    /// # Panics
    /// 如果表为空则报错.
    fn partition(&mut self) -> (T, Self) {
        let flag = self.pop_front().unwrap();
        let mut rhs = Self::default();
        let mut rhs_cursor = rhs.cursor_front_mut();
        let mut lhs_cursor = self.cursor_front_mut();
        while let Some(elem) = lhs_cursor.peek() {
            if *elem >= flag {
                // 已判空, 故可直接`unwrap`.
                rhs_cursor.insert_after_as_current(lhs_cursor.remove_current().unwrap());
            } else {
                lhs_cursor.move_next();
            }
        }
        (flag, rhs)
    }

    /// (按递增序)排序.
    // 习题 2.3.6
    pub fn sort(&mut self) {
        if !self.is_empty() {
            let (flag, mut rhs) = self.partition();
            self.sort();
            rhs.sort();
            rhs.push_front(flag);
            self.append(rhs);
        }
    }

    /// 删除内容在[a, b)之间的结点.
    // 习题 2.3.7
    pub fn delete_between(&mut self, a: &T, b: &T) {
        if *a < *b {
            let mut cursor = self.cursor_front_mut();
            while let Some(elem) = cursor.as_cursor().peek() {
                if *a <= *elem && *elem < *b {
                    cursor.remove_current();
                } else {
                    cursor.move_next();
                }
            }
        }
    }

    /// 串匹配. 若匹配, 则返回最近匹配的位置; 否则返回`None`.
    /// # 目前的实现
    /// 朴素匹配算法.
    // 习题 2.3.16
    pub fn find(&self, target: &Self) -> Option<Cursor<T>> {
        let mut cur = self.cursor_front();
        if self.is_empty() && target.is_empty() {
            return Some(cur);
        }
        while cur.peek().is_some() {
            let mut pcur = cur;
            let mut tcur = target.cursor_front();
            while let (Some(pe), Some(te)) = (pcur.peek(), tcur.peek()) {
                if *pe != *te {
                    break;
                }
                pcur.move_next();
                tcur.move_next();
            }
            if tcur.peek().is_none() {
                return Some(cur);
            }
            cur.move_next();
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_last(data: Vec<usize>, n: usize) {
            let list = LinkedList::from(data.clone());
            if 1 <= n && n <= data.len() {
                assert_eq!(list.last(n), Some(&data[data.len() - n]));
            } else {
                assert_eq!(list.last(n), None);
            }
        }

        #[test]
        fn test_find(data: String, pattern: String) {
            let list = LinkedList::from(Vec::from(data.clone()));
            let cursor = list.find(&LinkedList::from(Vec::from(pattern.clone())));
            let idx = match cursor.map(|cur| cur.index()) {
                Some(Some(idx)) => Some(idx),
                Some(None) => Some(0),
                _ => None,
            };
            prop_assert_eq!(idx, data.find(&pattern));
        }

        #[test]
        fn test_dedup(mut data: Vec<i64>) {
            let mut list = LinkedList::from(data.clone());
            list.dedup();
            data.dedup();
            assert_eq!(list, data);
        }

        #[test]
        fn test_delete_between(data: Vec<isize>, a: isize, b: isize) {
            let mut list = LinkedList::from(data);
            list.delete_between(&a, &b);
            for v in list.iter() {
                prop_assert!(!(a <= *v && *v < b));
            }
        }

        #[test]
        fn test_sort(mut data: Vec<isize>) {
            let mut list = LinkedList::from(data.clone());
            list.sort();
            data.sort_unstable();
            prop_assert_eq!(data, list);
        }

        #[test]
        fn test_reverse(mut data: Vec<isize>) {
            let mut list: LinkedList<_> = data.clone().into();
            list.reverse();
            data.reverse();
            prop_assert_eq!(list, data);
        }

        #[test]
        fn test_basic(data: Vec<isize>) {
            let list: LinkedList<_> = data.clone().into();
            prop_assert_eq!(data, Vec::from(list));
        }

        #[test]
        fn test_delete_all(data: Vec<isize>) {
            let mut list: LinkedList<_> = data.clone().into();
            if !data.is_empty() {
                let target = data[0];
                list.delete_all(&target);
                let mut iter = list.iter_mut();
                for v in data {
                    if v != target {
                        prop_assert_eq!(Some(v), iter.next().copied())
                    }
                }
            } else {
                list.delete_all(&1);
            }
        }

        #[test]
        fn test_cursor_mut_insert_before(data: Vec<isize>) {
            // 逆序插入
            let mut list = LinkedList::default();
            let mut cursor = list.cursor_front_mut();

            for v in data.iter().rev() {
                prop_assert_eq!(cursor.insert_before_as_current(*v), None);
            }

            prop_assert_eq!(&list, &data);

            // 通过`insert_before`插入.
            let mut list = LinkedList::default();
            let mut cursor = list.cursor_front_mut();

            if !data.is_empty() {
                prop_assert_eq!(cursor.insert_before(data[data.len() - 1]), None);
            }

            if data.len() > 1 {
                for v in data[0..(data.len() - 1)].iter() {
                    prop_assert_eq!(cursor.insert_before(*v), None);
                }
            }

            prop_assert_eq!(&list, &data);
        }

        #[test]
        fn test_cursor_mut_insert_after(data: Vec<isize>) {
            // 顺序插入(通过`insert_after_as_current`)
            let mut list = LinkedList::default();
            let mut cursor = list.cursor_front_mut();

            // 不变式: 游标始终指向尾结点的后继. 因此调用`insert_after`将会始终在末尾插入新元素.
            // 开始时表为空, `cursor.prev`为头结点(可看作尾结点), 因此游标指向尾结点的后继`None`.
            for v in data.iter() {
                // 在尾结点的后继之后插入新的元素(将会直接插入作为尾结点的后继), 插入后游标指向新插入的结点(即新的尾结点).
                prop_assert_eq!(cursor.insert_after_as_current(*v), None);
            }

            // 插入到`None`的后继的后面将会失败.
            cursor.move_next();
            cursor.move_next();
            prop_assert_eq!(cursor.insert_after(1), Some(1));
            prop_assert_eq!(&list, &data);

            // 通过`insert_after`插入.
            let mut list = LinkedList::default();
            let mut cursor = list.cursor_front_mut();

            if !data.is_empty() {
                prop_assert_eq!(cursor.insert_after(data[0]), None);
            }

            if data.len() > 1 {
                for v in data[1..].iter().rev() {
                    prop_assert_eq!(cursor.insert_after(*v), None);
                }
            }
            prop_assert_eq!(list, data);
        }

        #[test]
        fn test_pop_min(mut data: Vec<isize>) {
            let mut list: LinkedList<_> = data.clone().into();
            let min = list.pop_min();
            let mut min_idx = None;
            if let Some(min) = min {
                for (idx, v) in data.iter().enumerate() {
                    prop_assert!(min <= *v);
                    if min_idx.is_none() && min == *v {
                        min_idx = Some(idx);
                    }
                };
            } else {
                prop_assert!(data.is_empty());
            }
            if let Some(idx) = min_idx {
                data.remove(idx);
            }

            prop_assert_eq!(list, data);
        }
    }

    #[test]
    fn test_cursor() {
        let mut list: LinkedList<_> = vec![1, 2, 3, 4, 5].into();
        let mut cursor = list.cursor_front_mut();
        cursor.move_next();
        assert_eq!(cursor.as_cursor().peek(), Some(&2));
        let mut c1 = cursor.as_cursor();
        c1.move_next();
        assert_eq!(c1.peek(), Some(&3));
        let mut c1 = list.cursor_front();
        let mut c2 = list.cursor_front();
        assert_eq!(c2.peek(), Some(&1));
        c1.move_next();
        assert_eq!(c1.peek(), Some(&2));
        c2.move_next();
        assert_eq!(c2.peek(), Some(&2));
        let mut c3 = list.cursor_front();
        assert_eq!(c3.peek(), Some(&1));
        c3.move_next();
        assert_eq!(c1.peek(), c3.peek());
    }
}
