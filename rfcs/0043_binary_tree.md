- 特性名称: `binary_tree`
- 提案创建日期: 2020-10-07
- RFC PR: [#0043](https://github.com/Nouzan/my-algo-rs/pull/0043)
- 关联 Issue: [#0000](https://github.com/Nouzan/my-algo-rs/issues/0000)

# 概述

作为树系列的第一份RFC，我们将会对二叉树结构进行总结和抽象. 首先，我们定义了`BinTree`特质和`BinTreeMut`特质，用于描述一个二叉树应该有哪些“特质”；接下来，我们给出了一个具体的顺序二叉树实现的例子`VecBinaryTree`；最后，我们描述了该如何去对二叉树性质进行测试.

# 动机

为了支持树结构算法(ch4)，我们需要引入一些描述树结构的定义. 作为树系列的第一份RFC，它的目的是描述*二叉树*.

# 指南级别描述

## 二叉树

### 定义与基本性质

**树**

树是没有回路的连通图. 对于有`n`个结点的树(不为空树)，它恰有`n-1`条边.

**二叉树**

每个结点的度至多为`3`的树. 若二叉树不为空树其中必存在结点，它的度数小于`3`，我们把其中一个这样的结点选作**根**. 对于度数小于`3`的结点，作为连通图它们的度数只可能是`1`或`2`，我们把不是根的的度数为`1`的结点叫做**叶子**；除**叶子**以外的结点，叫做**内部结点**.

**二叉树的祖先、父母与孩子**

选定**根**以后，由树的无环性知每一个结点到**根**的路径是唯一的. 我们把每个结点到**根**的路径上的每个结点叫做该结点的**祖先**(以一个结点为祖先的结点，称为前者的**后代**)，在这条路径上与它关联的结点(唯一)叫做它的**父母**，不在这条路径上的与它关联的结点叫做它的**孩子**，我们定义一个结点的**孩子**的个数为**孩子度**(清华版数据结构与算法中对树结点度的定义实为**孩子度**).

对于二叉树而言，每个结点的**孩子度**只可能是`0`，`1`或`2`，内部结点的孩子度为`1`或`2`，叶子的孩子度为`0`.

**子树与有序性**

一个结点的所有**后代**及相关联的边构成的子图也是一棵树，子树的根通常选为该结点. 我们把它称为以该结点为根的子树.

如果我们规定每个结点的孩子的顺序，那么我们可以分别将它们称作二叉树的**左孩子**和**右孩子**(若有).

**深度与高度**

每个结点到根所经过的边的条数称作**深度**，树中所有结点的深度的最大值称为树的**高度**.

**真二叉树**

若不含**孩子度**为`1`的结点，则该二叉树称为**真二叉树**.

**树与二叉树的转化**

任何一棵(有序)树可以通过*长子、兄弟表示法*转化为一颗二叉树.

## 二叉树特质：`BinTree` trait和`BinTreeMut` trait

我们的设计原则是充分利用**树的递归性质**：树的结点也可以看作是以该结点为根的子树. 因此，树既有作为一个树结点(容器)的性质，又有作为树的性质.

**二叉树特质**

二叉树特质规定了一棵二叉树的*内容*类型为`BinTree::Elem`. 一棵二叉树的结构性特征由*游标*类型`Node`和`NodeMut`规定，它们分别实现了结点特质`BinTreeNode`和`BinTreeNodeMut`(见下方)，这里我们要求**不可变游标**还是一个`Clone`，以利用其共享访问权的性质.

对于不可变的二叉树，我们可以利用其**不可变游标**来实现各类*遍历迭代器*. 如目前实现了的层序遍历迭代器`InOrderIter`：它充分利用**不可变游标**的`Clone`特质，利用`VecDeque`作为队列实现了通用的*非递归版的二叉树层序遍历算法*.

```rust
/// 不可变二叉树特质.
pub trait BinTree {
    /// 内容类型.
    type Elem;

    /// 不可变结点类型
    type Node<'a, T: 'a>: BinTreeNode<Elem = T> + Clone;

    /// 是否为空树.
    fn is_empty(&self) -> bool {
        self.cursor().as_ref().is_none()
    }

    /// 创建一个只读结点游标.
    fn cursor<'a>(&'a self) -> Self::Node<'a, Self::Elem>;

    /// 层序遍历迭代器.
    fn in_order_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self::Elem> + 'a> {
        let mut queue = VecDeque::new();
        if self.cursor().as_ref().is_some() {
            queue.push_back(self.cursor())
        }
        Box::new(InOrderIter {
            queue,
            marker: PhantomData::default(),
        })
    }
}

/// 可变二叉树特质.
pub trait BinTreeMut: BinTree {
    /// 可变结点类型
    type NodeMut<'a, T: 'a>: BinTreeNodeMut<Elem = T>;

    /// 创建一个可变结点游标.
    fn cursor_mut<'a>(&'a mut self) -> Self::NodeMut<'a, Self::Elem>;
}
```

**二叉树结点特质**

结点特质是对二叉树*位置*以及该位置上的*内容访问权*的抽象，我们也把这种抽象称作*游标*.

作为*内容访问权*的抽象，我们能获得结点所指元素内容的只读引用(`BinTreeNode::as_ref`)或可变引用(`BinTreeNodeMut::as_mut`)，同时能获得与该结点直接关联的结点(左孩子和右孩子)的内容的只读引用(`BinTreeNode::left/right`)和可变引用(`BinTreeNodeMut::left_mut/right_mut`).

而作为*位置*的抽象，我们可以通过`BinTreeNode::move_left`和`BinTreeNode::move_right`在树上进行*相对地移动*(或称为*循位置访问*)，同时我们可以对直接关联的结点位置进行插入(`BinTreeNodeMut::insert_as_*`, `BinTreeNodeMut::append_*`)和删除(`BinTreeNodeMut::take_*`, `BinTreeNodeMut::into_inner`).

**树的递归性质**集中体现在了`BinTreeNode`以及`BinTreeNodeMut`这两个特质中：它们要求自身同时分别是一个`BinTree`和`BinTreeMut`(在实际使用过程中，我们会递归地将`BinTree::Node`和`BinTree::NodeMut`实现为结点类型自身).

```rust
/// 不可变二叉树结点特质.
pub trait BinTreeNode: BinTree {
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
}

/// 可变二叉树结点特质.
pub trait BinTreeNodeMut: BinTreeNode + BinTreeMut {
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
```

**例子**

实现的例子请参考**文档级别描述**中的`VecBinaryTree`。

# 文档级别描述

## 实现

特质的实现参考**指南级别描述**.

## 例子与验证

### 顺序二叉树：`VecBinaryTree` struct

// TODO

### 测试二叉树

// TODO

# 缺点

// TODO

# 待解决的问题

暂无

# 替代方案

暂无