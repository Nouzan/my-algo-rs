use super::{DoublyLinkedBinaryTree, Link, Node, NodePosi};
use crate::ch4::{BinTree, BinTreeCursor, BinTreeCursorMut, MoveParentCursor, MoveParentCursorMut};

/// 不可变游标.
/// # Safety
/// 游标类型的安全性和正确性关键在于以下几个不变式:
///
/// 1. 保持树的基本不变式：
///     1) `root`指针非空，所指结点存在，且根结点(若存在)是它的左孩子(有根性).
///     2) 对于树中任何结点a和b，它们要么不关联；要么(不失一般性地)a是b的左孩子异或右孩子，则b必然是a的父母，a没有其它父母且a不能是其它结点的孩子(完备性).
///     3) 树中任何结点至多只有2个孩子，任何非哨兵根结点有且只有1个父母(计数性质).
///
/// 2. 树中所有关联指针(包括哨兵`root`)都是合法的. 即我们可以安全地将它转换为一个`Box<Node<T>>`.
///
/// 3. 树中所有关联指针都是局部共享的. 即指针所对应的结点`Box<Node<T>>`只能通过指针共享给游标或与之关联的结点而不会重复.
///
/// 4. 任何非`root`指针所对应的结点都是完整的，它的`parent`必然非空，它的`elem`必然非空. `root`指针`parent`和`elem`必然为空.
///
/// 5. 游标指针的`parent`指针始终指向`tree`中的一个结点(可以是`root`)，且始终是当前所指结点的父母，左右性由`is_left`表征.
pub struct Cursor<'a, T> {
    parent: NodePosi<T>,
    is_left: bool,
    tree: &'a DoublyLinkedBinaryTree<T>,
}

/// 可变游标.
///
/// # Safety
/// 关于可变游标，我们引入一个新的不变式:
/// 6. 互斥性: 受限于生命期`'a`，我们不可能安全地拥有多个可变游标，而在可变游标的实现中，我们也应该保持这一点.
pub struct CursorMut<'a, T> {
    parent: NodePosi<T>,
    is_left: bool,
    tree: &'a mut DoublyLinkedBinaryTree<T>,
}

/// 返回所指结点的父母指针.
/// # Safety
/// `link`必须是合法的[2].
unsafe fn parent<T>(link: Link<T>) -> Link<T> {
    link.and_then(|posi| posi.as_ref().parent)
}

/// 返回所指结点的左孩子指针.
/// # Safety
/// `link`必须是合法的[2].
unsafe fn left<T>(link: Link<T>) -> Link<T> {
    link.and_then(|posi| posi.as_ref().left)
}

/// 返回所指结点的右孩子指针.
/// # Safety
/// `link`必须是合法的[2].
unsafe fn right<T>(link: Link<T>) -> Link<T> {
    link.and_then(|posi| posi.as_ref().right)
}

/// 返回所指结点的内容的引用.
/// # Safety
/// `link`必须是合法的[2]，且生命期`'a`内不存在其它共享的可变引用(只读共享).
unsafe fn elem<'a, T>(link: Link<T>) -> Option<&'a T> {
    link.and_then(|posi| (*posi.as_ptr()).elem.as_ref())
}

/// 返回所指结点的内容的可变引用.
/// # Safety
/// `link`必须是合法的[2]，且生命期`'a`内不存在其它共享的引用(可写互斥).
unsafe fn elem_mut<'a, T>(link: Link<T>) -> Option<&'a mut T> {
    link.and_then(|posi| (*posi.as_ptr()).elem.as_mut())
}

// 注意到我们只有树的只读引用，因此树的所有不变式均保持.
// 只需要验证`parent`域的不变式[5]即可.
impl<'a, T> Cursor<'a, T> {
    /// 提供一个树的引用，返回一个只读游标.
    pub fn new(tree: &'a DoublyLinkedBinaryTree<T>) -> Self {
        // 初始时，`parent`指向`tree.root`，因此是树中的一个结点.
        // 游标当前所指结点是树根，根据树的不变式[0]，它的父母的确是`root`指针.
        // 而根据树的不变式[0]，根结点始终是`root`指针所指结点的左孩子，
        // 因此`is_left`正确表征了这一点. 所以方法保持了不变式[5].
        Self {
            parent: tree.root,
            is_left: true,
            tree,
        }
    }

    /// 返回当前所指结点的一个指针.
    ///
    /// 返回的是`self.parent`所指结点的孩子指针，根据不变式[5]和[2]知它是合法的，且的确是当前所指结点.
    fn current_link(&self) -> Link<T> {
        // Safety: 根据不变式[5]，`self.parent`是树中一个结点的指针.
        // 再根据不变式[2]，树中指针都是合法的，因此解引用是安全的.
        unsafe {
            if self.is_left {
                self.parent.as_ref().left
            } else {
                self.parent.as_ref().right
            }
        }
    }

    /// 判断当前所指结点是否为树根.
    ///
    /// 当且仅当当前所指结点的父母是`root`指针(树的不变式[0]).
    fn is_root(&self) -> bool {
        self.parent == self.tree.root
    }
}

impl<'a, T> Clone for Cursor<'a, T> {
    fn clone(&self) -> Self {
        Self {
            parent: self.parent,
            is_left: self.is_left,
            tree: self.tree,
        }
    }
}

impl<'a, T> BinTreeCursor<'a> for Cursor<'a, T> {
    type Elem = T;

    fn is_parent(&self, other: &Self) -> bool {
        // 这里要求`self`和`other`所对应的是同一棵树.
        // 根据不变式[5]，`parent`为当前所指结点的父母指针.
        // 再根据树中指针的局部共享性[3]，下式成立当且仅当
        // `other`所指结点的确是`self`所指结点的父母.
        other.current_link() == Some(self.parent)
    }

    fn as_ref(&self) -> Option<&Self::Elem> {
        // Safety: `current_link`返回的指针(若有)是合法的.
        unsafe { elem(self.current_link()) }
    }

    fn left(&self) -> Option<&Self::Elem> {
        // Safety: `current_link`返回的指针(若有)是合法的.
        unsafe { elem(left(self.current_link())) }
    }

    fn right(&self) -> Option<&Self::Elem> {
        // Safety: `current_link`返回的指针(若有)是合法的.
        unsafe { elem(right(self.current_link())) }
    }

    fn move_left(&mut self) {
        // `current_link`返回的指针(若有)是合法的，因此调用后`parent`指针依然是合法的.
        // 而方法的语义是移动至当前所指结点(若有)的左孩子，因此调用后当前所指结点变为原来
        // 结点的孩子，而`parent`指针是原来结点的指针，因此是当前所指结点的父母，
        // 且左右孩子性由`is_left`正确表征. 因此保持了不变式[5].
        if let Some(posi) = self.current_link() {
            self.parent = posi;
            self.is_left = true;
        }
    }

    fn move_right(&mut self) {
        // 同`move_left`.
        if let Some(posi) = self.current_link() {
            self.parent = posi;
            self.is_left = false;
        }
    }

    fn into_ref(self) -> Option<&'a Self::Elem>
    where
        Self: Sized,
    {
        // Safety: `current_link`返回的指针(若有)是合法的.
        unsafe { elem(self.current_link()) }
    }
}

impl<'a, T> MoveParentCursor<'a> for Cursor<'a, T> {
    fn move_parent(&mut self) {
        if !self.is_root() {
            // 根据不变式[5]和关联指针所指结点的完整性[4]，
            // `parent`所指结点既然不是`root`指针
            // (`is_root`为`true`当且仅当`parent`是`root`指针)
            // 则必然有父母指针非空，再根据关联指针的合法性[2]，
            // `parent`的父母指针必然是合法的.
            // 该方法的语义是移动至当前结点的父母，因此调用后当前结点变为
            // 原来结点的父母，而`parent`指针被移动到了原来结点的父母的父母，
            // 因此`parent`的确是调用后当前结点的父母.
            // 容易验证，末尾对`is_left`的赋值能正确表征调用后的左右孩子性.
            // 因此方法保持了不变式[5].
            //
            // Safety: `current`是`parent`指针，根据不变式[5]是合法的.
            unsafe {
                let current = Some(self.parent);
                self.parent = parent(current).unwrap();
                self.is_left = !((self.current_link() == current) ^ self.is_left)
            }
        }
    }

    fn parent(&self) -> Option<&Self::Elem> {
        if !self.is_root() {
            // Safety: 根据不变式[5]，`parent`指针是合法的.
            unsafe { elem(Some(self.parent)) }
        } else {
            None
        }
    }
}

// `new`及所有只读函数的安全性与正确性跟`Cursor`是一致的.
impl<'a, T> CursorMut<'a, T> {
    pub fn new(tree: &'a mut DoublyLinkedBinaryTree<T>) -> Self {
        Self {
            parent: tree.root,
            is_left: true,
            tree,
        }
    }

    fn current_link(&self) -> Link<T> {
        unsafe {
            if self.is_left {
                self.parent.as_ref().left
            } else {
                self.parent.as_ref().right
            }
        }
    }

    fn is_root(&self) -> bool {
        self.parent == self.tree.root
    }
}

impl<'a, T> BinTreeCursor<'a> for CursorMut<'a, T> {
    type Elem = T;

    fn is_parent(&self, other: &Self) -> bool {
        other.current_link() == Some(self.parent)
    }

    fn as_ref(&self) -> Option<&Self::Elem> {
        unsafe { elem(self.current_link()) }
    }

    fn left(&self) -> Option<&Self::Elem> {
        unsafe { elem(left(self.current_link())) }
    }

    fn right(&self) -> Option<&Self::Elem> {
        unsafe { elem(right(self.current_link())) }
    }

    fn move_left(&mut self) {
        if let Some(posi) = self.current_link() {
            self.parent = posi;
            self.is_left = true;
        }
    }

    fn move_right(&mut self) {
        if let Some(posi) = self.current_link() {
            self.parent = posi;
            self.is_left = false;
        }
    }

    fn into_ref(self) -> Option<&'a Self::Elem>
    where
        Self: Sized,
    {
        unsafe { elem(self.current_link()) }
    }
}

impl<'a, T> MoveParentCursor<'a> for CursorMut<'a, T> {
    fn move_parent(&mut self) {
        if !self.is_root() {
            // 非根则必有父母.
            unsafe {
                let current = Some(self.parent);
                self.parent = parent(current).unwrap();
                self.is_left = !((self.current_link() == current) ^ self.is_left)
            }
        }
    }

    fn parent(&self) -> Option<&Self::Elem> {
        if !self.is_root() {
            unsafe { elem(Some(self.parent)) }
        } else {
            None
        }
    }
}

impl<'a, T> BinTreeCursorMut<'a> for CursorMut<'a, T> {
    type SubTree = DoublyLinkedBinaryTree<T>;

    fn as_mut(&mut self) -> Option<&mut Self::Elem> {
        // Safety: `current_link`是合法的.
        // 另外由于互斥性[6]，不会安全地存在其它的对结点内容的可变引用.
        unsafe { elem_mut(self.current_link()) }
    }

    fn left_mut(&mut self) -> Option<&mut Self::Elem> {
        // Safety: `current_link`是合法的，它的左孩子指针(若有)也是合法的.
        // 另外由于互斥性[6]，不会安全地存在其它的对结点内容的可变引用.
        unsafe { elem_mut(left(self.current_link())) }
    }

    fn right_mut(&mut self) -> Option<&mut Self::Elem> {
        // Safety: `current_link`是合法的，它的右孩子指针(若有)也是合法的.
        // 另外由于互斥性[6]，不会安全地存在其它的对结点内容的可变引用.
        unsafe { elem_mut(right(self.current_link())) }
    }

    fn move_succ_and_split_mut(&mut self) -> (Option<&mut Self::Elem>, Option<&mut Self::Elem>) {
        // Safety: `move_left`和`move_right`的确不会改变树，也不会移动树的结点.
        // 另外由于互斥性[6]，不会安全地存在其它的对结点内容的可变引用，
        // 且方法的默认实现会确保返回的两个结点内容是不同的.
        unsafe { self.move_succ_and_split_mut_unchecked() }
    }

    fn insert_as_root(&mut self, elem: Self::Elem) -> Option<Self::Elem> {
        if self.is_empty_subtree() {
            // 这里创建了一个`Box<Node<T>`，并将它的父母域置为`parent`.
            let mut node = Node::new(elem);
            node.parent = Some(self.parent);
            // 经过判断，作为`parent`的孩子的当前结点不存在，通过`is_left`我们能判断
            // 究竟是哪一个孩子(即当前结点)是不存在的(由不变式[5]保证). 因此，我们可以
            // 直接用新建结点的指针去取代相应关联域(`left`或`right`)的内容，而不会造成
            // 内存泄露. 注意，新指针来源于`Box::leak`，因此在起码`T`类型的生命期内，
            // 新指针都是合法的，而树的生命期受限于`T`的生命期，因此在树的生命期内，新指针
            // 都是合法的，因此对`parent`而言，方法保持了树的关联指针的合法性[2]. 在方法调用后，
            // 新结点作为`parent`所指结点的孩子加入了树中，因此我们必须验证它的关联指针的合法性.
            // 而新结点的非空关联指针只有它的父母指针，而这正是合法的`parent`. 因此该方法对所有
            // 改动的结点均保持了合法性[2].
            // 容易验证，方法对`parent`和新结点均保持了树的性质[1]、局部共享性[3]和完整性[4].
            //
            // Safety: 由合法性[2]知，此处解引用是安全的.
            unsafe {
                if self.is_left {
                    self.parent.as_mut().left = Some(Node::leak(node));
                } else {
                    self.parent.as_mut().right = Some(Node::leak(node));
                }
            }
            None
        } else {
            Some(elem)
        }
    }

    fn insert_as_left(&mut self, elem: Self::Elem) -> Option<Self::Elem> {
        if let Some(mut current) = self.current_link() {
            if self.left().is_none() {
                // 类似于`insert_as_root`，新建结点作为当前结点的左孩子(必为空)被
                // 添加进树中，它的唯一关联指针指向它的父母即当前结点，是合法的[2].
                // 而被修改的当前结点(经检验存在)，它的左孩子指针被更新为刚刚通过`Box::leak`
                // 得到的新结点的指针，因此也是合法的[2]. 容易验证，方法保持了这两个结点
                // 的树的性质[1]、局部共享行[3]和完整性[4].
                //
                // Safety: 当前结点(经检验存在)的指针是合法的.
                let mut node = Node::new(elem);
                node.parent = Some(current);
                unsafe {
                    current.as_mut().left = Some(Node::leak(node));
                }
                None
            } else {
                Some(elem)
            }
        } else {
            Some(elem)
        }
    }

    fn insert_as_right(&mut self, elem: Self::Elem) -> Option<Self::Elem> {
        if let Some(mut current) = self.current_link() {
            if self.right().is_none() {
                // Safety: 与`insert_as_right`对称.
                let mut node = Node::new(elem);
                node.parent = Some(current);
                unsafe {
                    current.as_mut().right = Some(Node::leak(node));
                }
                None
            } else {
                Some(elem)
            }
        } else {
            Some(elem)
        }
    }

    fn take(&mut self) -> Self::SubTree {
        let mut tree = DoublyLinkedBinaryTree::new();
        unsafe {
            // Safety: `parent`是合法的.
            if let Some(posi) = if self.is_left {
                self.parent.as_mut().left.take()
            } else {
                self.parent.as_mut().right.take()
            } {
                // Safety: 由`parent`的合法性[2]知它的孩子若有则必然是合法的，
                // 且它以及它的子树不在空树`tree`中且合法.
                tree.replace_root_node(Some(posi));
            }
            // 方法调用后，原来的树将会失去以当前结点为根的子树(若有)，且所有关联都会被解除.
            // 因此自然保持了所有不变式.
        }
        tree
    }

    fn append(&mut self, mut other: Self::SubTree) {
        if self.is_empty_subtree() {
            unsafe {
                // Safety: 用`None`替换`other`的根结点是平凡的.
                let root = other.replace_root_node(None);
                // Safety: `root`来源于一棵树，它满足相应的不变式，因此它是合法的.
                if let Some(mut posi) = root {
                    posi.as_mut().parent = Some(self.parent);
                }
                // Safety: 根据不变式`parent`是合法的.
                if self.is_left {
                    self.parent.as_mut().left = root;
                } else {
                    self.parent.as_mut().right = root;
                }
                // 方法调用后，当前所指的空子树会被替换为以`other`的根结点的树(若存在)，
                // 显然满足所有的不变式.
            }
        } else {
            panic!("子树不为空!")
        }
    }

    fn take_left(&mut self) -> Option<Self::SubTree>
    where
        Self::SubTree: Sized,
    {
        unsafe {
            if let Some(mut posi) = self.current_link() {
                let mut tree = DoublyLinkedBinaryTree::new();
                // Safety: `current_link`返回的指针是合法的.
                let posi = posi.as_mut().left.take();
                // Safety: `posi`来源于当前结点的左子树，根据不变式它满足所有需要的性质.
                tree.replace_root_node(posi);
                // 方法移除了当前结点的左子树，依然保持所有不变式.
                Some(tree)
            } else {
                None
            }
        }
    }

    fn take_right(&mut self) -> Option<Self::SubTree>
    where
        Self::SubTree: Sized,
    {
        unsafe {
            if let Some(mut posi) = self.current_link() {
                // 与`take_left`对称.
                let mut tree = DoublyLinkedBinaryTree::new();
                let posi = posi.as_mut().right.take();
                tree.replace_root_node(posi);
                Some(tree)
            } else {
                None
            }
        }
    }

    fn append_left(&mut self, mut other: Self::SubTree) {
        if let Some(mut posi) = self.current_link() {
            unsafe {
                if left(Some(posi)).is_none() {
                    // Safety: 用`None`替换树根是平凡的.
                    let root = other.replace_root_node(None);
                    // Safety: `root`来源于另一棵树，它的不变式保证了`root`的合法性.
                    if let Some(mut p) = root {
                        p.as_mut().parent = Some(posi);
                    }
                    posi.as_mut().left = root;
                // 这里验证过当前结点的左孩子为空，因此不会违反任何不变式.
                } else {
                    panic!("左子树不为空!")
                }
            }
        } else {
            panic!("子树为空!")
        }
    }

    fn append_right(&mut self, mut other: Self::SubTree) {
        if let Some(mut posi) = self.current_link() {
            // 同`append_left`对称.
            unsafe {
                if right(Some(posi)).is_none() {
                    let root = other.replace_root_node(None);
                    if let Some(mut p) = root {
                        p.as_mut().parent = Some(posi);
                    }
                    posi.as_mut().right = root;
                } else {
                    panic!("右子树不为空!")
                }
            }
        } else {
            panic!("子树为空!")
        }
    }

    fn into_mut(self) -> Option<&'a mut Self::Elem>
    where
        Self: Sized,
    {
        // Safety: `current_link`返回的指针是合法的.
        // 且根据互斥性[6]，`'a`内不存在其它的可变引用，这里也依然保持了互斥性.
        unsafe { elem_mut(self.current_link()) }
    }

    fn into_inner(mut self) -> Option<Self::Elem> {
        // 根据局部共享性，当前所指结点只会被共享到它的父母和子树，而与父母的关联被`take`切断
        // 子树在方法结束后会被`drop`，因此方法结束后不会存在其它的指向当前结点内容的引用或指针.

        // 由于`into_root`会返回根结点(包含着左右子树的引用)，
        // 但我们这里并不打算手动释放左右子树，因此提前将它们摘除.
        self.take_left();
        self.take_right();
        let tree = self.take();
        tree.into_root().and_then(|node| node.elem)
    }
}

impl<'a, T> MoveParentCursorMut<'a> for CursorMut<'a, T> {
    fn parent_mut(&mut self) -> Option<&mut Self::Elem> {
        if !self.is_root() {
            // Safety: `parent`指针是合法的.
            // 另外由于互斥性[6]，不会安全地存在其它的对结点内容的可变引用.
            unsafe { elem_mut(Some(self.parent)) }
        } else {
            None
        }
    }
}

impl<'a, T> BinTree for CursorMut<'a, T> {
    type Elem = T;
    type Cursor<'b, E: 'b> = Cursor<'b, E>;

    fn cursor(&self) -> Self::Cursor<'_, Self::Elem> {
        Cursor {
            parent: self.parent,
            is_left: self.is_left,
            tree: self.tree,
        }
    }
}
