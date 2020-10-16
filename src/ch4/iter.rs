use super::BinTreeCursor;
use std::collections::VecDeque;
use std::marker::PhantomData;

/// 层序遍历迭代器结构.
pub struct InOrderIter<'a, Cursor> {
    queue: VecDeque<Cursor>,
    marker: PhantomData<&'a Cursor>,
}

impl<'a, Cursor> InOrderIter<'a, Cursor> {
    pub fn new(root: Option<Cursor>) -> Self {
        let mut queue = VecDeque::new();
        if let Some(root) = root {
            queue.push_back(root);
        }
        InOrderIter {
            queue,
            marker: PhantomData::default(),
        }
    }
}

impl<'a, T: 'a, Cursor: BinTreeCursor<'a, Elem = T> + Clone> Iterator for InOrderIter<'a, Cursor> {
    type Item = &'a Cursor::Elem;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cursor) = self.queue.pop_front() {
            let (left, right) = cursor.split();
            if let Some(left) = left {
                self.queue.push_back(left);
            }
            if let Some(right) = right {
                self.queue.push_back(right);
            }
            cursor.into_ref()
        } else {
            None
        }
    }
}

/// 前序遍历迭代器结构.
pub struct PreOrderIter<'a, Cursor> {
    current: Option<Cursor>,
    stack: Vec<Cursor>,
    marker: PhantomData<&'a Cursor>,
}

impl<'a, Cursor> PreOrderIter<'a, Cursor> {
    pub fn new(root: Option<Cursor>) -> Self {
        Self {
            current: root,
            stack: Vec::new(),
            marker: PhantomData::default(),
        }
    }
}

impl<'a, T: 'a, Cursor: BinTreeCursor<'a, Elem = T> + Clone> Iterator for PreOrderIter<'a, Cursor> {
    type Item = &'a Cursor::Elem;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current.take() {
            self.stack.push(current.clone());
            if current.left().is_some() {
                let mut next = current.clone();
                next.move_left();
                self.current = Some(next);
            } else {
                while let Some(mut old) = self.stack.pop() {
                    if old.right().is_some() {
                        old.move_right();
                        self.current = Some(old);
                        break;
                    }
                }
            }
            current.into_ref()
        } else {
            None
        }
    }
}

/// 中序遍历迭代器结构.
pub struct MidOrderIter<'a, Cursor> {
    current: Option<Cursor>,
    stack: Vec<Cursor>,
    marker: PhantomData<&'a Cursor>,
}

impl<'a, Cursor: BinTreeCursor<'a> + Clone> MidOrderIter<'a, Cursor> {
    fn push_left_chain(&mut self, current: &mut Cursor) {
        while !current.is_empty_subtree() {
            self.stack.push(current.clone());
            current.move_left();
        }
    }

    pub fn new(mut root: Cursor) -> Self {
        let mut iter = Self {
            current: None,
            stack: Vec::new(),
            marker: PhantomData::default(),
        };
        iter.push_left_chain(&mut root);
        iter.current = iter.stack.pop();
        iter
    }
}

impl<'a, T: 'a, Cursor: BinTreeCursor<'a, Elem = T> + Clone> Iterator for MidOrderIter<'a, Cursor> {
    type Item = &'a Cursor::Elem;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current.take() {
            let mut right = current.clone();
            right.move_right();
            self.push_left_chain(&mut right);
            self.current = self.stack.pop();
            current.into_ref()
        } else {
            None
        }
    }
}

/// 后序遍历迭代器.
pub struct PostOrderIter<'a, Cursor> {
    stack: Vec<Cursor>,
    marker: PhantomData<&'a Cursor>,
}

impl<'a, Cursor: BinTreeCursor<'a> + Clone> PostOrderIter<'a, Cursor> {
    /// 寻找以栈顶为根的最高的左侧可见叶结点(HLVFL)，并将沿途结点及其右兄弟入栈(右兄弟优先入栈).
    fn find_hlvfl(&mut self) {
        // 不变式: 栈顶为子树中HLVFL在栈中的最近祖先, 次顶(若在子树中)为栈顶的父母或右兄弟.
        while let Some(top) = self.stack.last() {
            if top.is_leaf() {
                // 找到HLVFL
                break;
            } else {
                let (left, right) = top.split();
                if let Some(right) = right {
                    self.stack.push(right);
                }
                if let Some(left) = left {
                    self.stack.push(left);
                }
            }
        }
    }

    pub fn new(root: Cursor) -> Self {
        let mut iter = Self {
            stack: Vec::new(),
            marker: PhantomData::default(),
        };

        if !root.is_empty_subtree() {
            iter.stack.push(root);
            iter.find_hlvfl();
        }

        iter
    }
}

impl<'a, T: 'a, Cursor: BinTreeCursor<'a, Elem = T> + Clone> Iterator
    for PostOrderIter<'a, Cursor>
{
    type Item = &'a Cursor::Elem;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(hlvfl) = self.stack.pop() {
            if let Some(top) = self.stack.last() {
                // 判断栈顶是否为当前结点的父母，若是则它就是下一hlvfl
                // 若不是则它必为当前结点的右兄，它的子树还未被扫描，且hlvfl在它的子树中.
                if !hlvfl.is_parent(top) {
                    self.find_hlvfl();
                }
            }
            hlvfl.into_ref()
        } else {
            None
        }
    }
}
