use super::BinTree;
use super::BinTreeMut;
use super::MoveParentBinTree;

pub trait BinTreeCursor<'a>: BinTree {
    /// 是否为空树.
    fn is_empty_subtree(&self) -> bool {
        self.as_ref().is_none()
    }

    /// 是否为叶子结点.
    fn is_leaf(&self) -> bool {
        self.left().is_none() && self.right().is_none()
    }

    // /// 判断另一结点是否为当前结点的右孩子.
    // fn is_right_child(&self, other: &Self) -> bool;

    /// 判断另一结点是否为当前结点的父母.(`other`与`self`必须是同一树中的结点.)
    fn is_parent(&self, other: &Self) -> bool;

    /// 若为空树则返回`None`，否则返回当前结点(根)的内容的引用.
    fn as_ref(&self) -> Option<&Self::Elem>;

    /// 若为空树或不含左孩子则返回`None`，否则返回左孩子的内容的引用.
    fn left(&self) -> Option<&Self::Elem>;

    /// 若为空树或不含右孩子则返回`None`，否则返回右孩子的内容的引用.
    fn right(&self) -> Option<&Self::Elem>;

    /// 若为空树则`no-op`，否则变为左子树.
    fn move_left(&mut self);

    /// 若为空树则`no-op`，否则变为右子树.
    fn move_right(&mut self);

    /// 移动到当前结点的在子树中的直接中序后继，若为空树或无右子树则为`no-op`.
    fn move_succ(&mut self) {
        if self.right().is_some() {
            self.move_right();
            while self.left().is_some() {
                self.move_left();
            }
        }
    }

    /// 创建指向左右子树的游标. 若为空树，则返回`None`.
    fn split(&self) -> (Option<Self>, Option<Self>)
    where
        Self: Sized + Clone,
    {
        if self.is_empty_subtree() {
            (None, None)
        } else {
            let left = if self.left().is_some() {
                let mut child = self.clone();
                child.move_left();
                Some(child)
            } else {
                None
            };
            let right = if self.right().is_some() {
                let mut child = self.clone();
                child.move_right();
                Some(child)
            } else {
                None
            };

            (left, right)
        }
    }

    fn into_ref(self) -> Option<&'a Self::Elem>
    where
        Self: Sized;
}

pub trait BinTreeCursorMut<'a>: BinTreeCursor<'a> {
    type SubTree;

    /// 若为空树则返回`None`，否则返回当前结点(根)的内容的可变引用.
    fn as_mut(&mut self) -> Option<&mut Self::Elem>;

    /// 若为空树或不含左孩子则返回`None`，否则返回左孩子的内容的可变引用.
    fn left_mut(&mut self) -> Option<&mut Self::Elem>;

    /// 若为空树或不含右孩子则返回`None`，否则返回右孩子的内容的可变引用.
    fn right_mut(&mut self) -> Option<&mut Self::Elem>;

    /// 移动至当前结点在子树中的直接中序后继，并返回原来所指结点内容的可变引用以及后继所指结点内容的可变引用.
    /// 若子树为空或右子树为空，则为`no-op`.
    /// 若子树为空则返回`None`.
    /// 若后继不存在时返回`None`.
    fn move_succ_and_split_mut(&mut self) -> (Option<&mut Self::Elem>, Option<&mut Self::Elem>);

    /// (unsafe版本)移动至当前结点在子树中的直接中序后继，并返回原来所指结点的可变引用.
    /// 若子树为空或右子树为空，则为`no-op`，子树不为空时返回当前结点的可变引用.
    /// 若子树为空则返回`None`.
    /// # Safety
    /// `move_right`和`move_left`不会改变树，以及原结点内容的地址，而仅仅只是在树上移动.
    unsafe fn move_succ_and_split_mut_unchecked(
        &mut self,
    ) -> (Option<&mut Self::Elem>, Option<&mut Self::Elem>) {
        let current = self.as_mut().map(|node| node as *mut _);
        let mut succ = None;
        if self.right().is_some() {
            self.move_right();
            while self.left().is_some() {
                self.move_left();
            }
            if self.as_mut().map(|node| node as *mut _) != current {
                succ = self.as_mut();
            }
        }
        (current.and_then(|ptr: *mut Self::Elem| ptr.as_mut()), succ)
    }

    /// 插入一个元素作为根.
    /// 若不为空树，则是`no-op`并返回被插入的元素，
    /// 否则将元素作为根插入树中，并返回`None`.
    fn insert_as_root(&mut self, elem: Self::Elem) -> Option<Self::Elem>;

    /// 插入一个元素作为左孩子.
    /// - 若为空树或左孩子不为空则为`no-op`，并返回被插入的元素.
    /// - 否则元素将作为左孩子插入树中，并返回`None`.
    fn insert_as_left(&mut self, elem: Self::Elem) -> Option<Self::Elem>;

    /// 插入一个元素作为右孩子.
    /// - 若为空树或右孩子不为空则为`no-op`，并返回被插入的元素.
    /// - 否则元素将作为右孩子插入树中，并返回`None`.
    fn insert_as_right(&mut self, elem: Self::Elem) -> Option<Self::Elem>;

    /// 消耗整棵子树返回根的内容. 若为空树，则返回`None`.
    fn into_inner(self) -> Option<Self::Elem>;

    /// 把一棵树作为子树接入.
    /// 若当前子树不为空则报错.
    fn append(&mut self, other: Self::SubTree);

    /// 把一棵树作为左子树接入.
    /// # Panics
    /// 若左子树不为空则报错.
    fn append_left(&mut self, other: Self::SubTree);

    /// 把一棵树作为右子树接入.
    /// # Panics
    /// 若右子树不为空则报错.
    fn append_right(&mut self, other: Self::SubTree);

    /// 摘取整棵子树并返回. 若树为空则返回空树.
    fn take(&mut self) -> Self::SubTree;

    /// 摘取左子树并返回. 若树为空，则返回`None`，若子树为空，则返回空树.
    fn take_left(&mut self) -> Option<Self::SubTree>
    where
        Self::SubTree: Sized;

    /// 摘取右子树并返回. 若树为空，则返回`None`，若子树为空，则返回空树.
    fn take_right(&mut self) -> Option<Self::SubTree>
    where
        Self::SubTree: Sized;

    fn into_mut(self) -> Option<&'a mut Self::Elem>
    where
        Self: Sized;

    /// 顺时针旋转子树(提升左子树)，游标所指位置不变.
    ///
    /// 若当前结点或左孩子不存在则为`no-op`.
    fn zig(&mut self)
    where
        Self::SubTree: BinTreeMut,
    {
        if let Some(mut left_child) = self.take_left() {
            // 当前结点存在.
            if let Some(left_child_right) = {
                let tree = left_child.cursor_mut().take_right();
                tree
            } {
                // 左孩子存在.
                let mut tree = self.take();
                tree.cursor_mut().append_left(left_child_right);
                left_child.cursor_mut().append_right(tree);
                self.append(left_child);
            }
        }
    }

    /// 逆时针旋转子树(提升右子树)，游标所指位置不变.
    ///
    /// 若当前结点或左孩子不存在则为`no-op`.
    fn zag(&mut self)
    where
        Self::SubTree: BinTreeMut,
    {
        if let Some(mut right_child) = self.take_right() {
            // 当前结点存在.
            if let Some(right_child_left) = {
                let tree = right_child.cursor_mut().take_left();
                tree
            } {
                // 左孩子存在.
                let mut tree = self.take();
                tree.cursor_mut().append_right(right_child_left);
                right_child.cursor_mut().append_left(tree);
                self.append(right_child);
            }
        }
    }
}

pub trait MoveParentCursor<'a>: BinTreeCursor<'a> + MoveParentBinTree {
    fn move_parent(&mut self);
    fn parent(&self) -> Option<&Self::Elem>;
    /// 判断当前结点是否为左孩子.
    fn is_left_child(&self) -> bool;
}

pub trait MoveParentCursorMut<'a>: MoveParentCursor<'a> {
    fn parent_mut(&mut self) -> Option<&mut Self::Elem>;
}
