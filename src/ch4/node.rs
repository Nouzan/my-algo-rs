use super::BinTree;

/// 不可变二叉树游标特质.
pub trait BinTreeNode<'a> {
    /// 内容类型.
    type Elem;

    /// 是否为空树.
    fn is_empty_subtree(&self) -> bool {
        self.as_ref().is_none()
    }

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

/// 可变二叉树游标特质.
pub trait BinTreeNodeMut<'a>: BinTreeNode<'a> {
    type Tree: BinTree;

    /// 若为空树则返回`None`，否则返回当前结点(根)的内容的可变引用.
    fn as_mut(&mut self) -> Option<&mut Self::Elem>;

    /// 若为空树或不含左孩子则返回`None`，否则返回左孩子的内容的可变引用.
    fn left_mut(&mut self) -> Option<&mut Self::Elem>;

    /// 若为空树或不含右孩子则返回`None`，否则返回右孩子的内容的可变引用.
    fn right_mut(&mut self) -> Option<&mut Self::Elem>;

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

    /// 摘取左子树并返回. 若树为空，则返回`None`，若子树为空，则返回空树.
    fn take_left(&mut self) -> Option<Self::Tree>
    where
        Self::Tree: Sized;

    /// 摘取右子树并返回. 若树为空，则返回`None`，若子树为空，则返回空树.
    fn take_right(&mut self) -> Option<Self::Tree>
    where
        Self::Tree: Sized;

    /// 把一棵树作为左子树接入. 操作后`other`变为空树.
    /// # Panics
    /// 若左子树不为空则报错.
    fn append_left(&mut self, other: &mut Self);

    /// 把一棵树作为右子树接入. 操作后`other`变为空树.
    /// # Panics
    /// 若右子树不为空则报错.
    fn append_right(&mut self, other: &mut Self);

    /// 消耗整棵树返回根的内容. 若为空树，则返回`None`.
    fn into_inner(self) -> Option<Self::Elem>;
}

// /// 可冻结的可变二叉树游标特质.
// pub trait FrozenNodeMut<'a>: BinTreeNodeMut<'a> {
//     /// 冻结可变结点，并创建一个它的复制.
//     /// # Correctness
//     /// 冻结后应保证被冻结游标在树中的*结构特征*不变，
//     /// 即解除冻结后所指结点不变.
//     fn frozen(&self) -> Self;
// }
