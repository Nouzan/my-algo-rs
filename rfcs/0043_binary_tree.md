- 特性名称: `binary_tree`
- 提案创建日期: 2020-10-07
- RFC PR: [#0043](https://github.com/Nouzan/my-algo-rs/pull/0043)
- 关联 Issue: [#0000](https://github.com/Nouzan/my-algo-rs/issues/0000)

# 概述

作为树系列的第一份RFC，我们将会对二叉树结构进行总结和抽象. 首先，我们定义了`BinTree`特质和`BinTreeMut`特质，用于描述一个二叉树应该有哪些“特质”；接下来，我们给出了一个具体的顺序二叉树实现的例子`VecBinaryTree`；最后，我们描述了该如何去对二叉树性质进行测试.

# 动机

为了支持树结构算法(ch4)，我们需要引入一些描述树结构的定义. 作为树系列的第一份RFC，它的目的是描述*二叉树及其算法*.

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

**子树**

一个结点的所有**后代**及相关联的边构成的子图也是一棵树，子树的根通常选为该结点. 我们把它称为以该结点为根的子树.

**深度与高度**

每个结点到根所经过的边的条数称作**深度**，树中所有结点的深度的最大值称为树的**高度**.

**真二叉树**

若不含**孩子度**为`1`的结点，则该二叉树称为**真二叉树**.

**树与二叉树的转化**

任何一棵(有序)树可以通过*长子、兄弟表示法*转化为一颗二叉树.

## 二叉树特质：`BinTree` trait和`BinTreeMut` trait

我们的设计原则是充分利用树的递归性质：树的结点也可以看作是以该结点为根的子树. 因此，树既有作为一个树结点(容器)的性质，又有作为树的性质.

**不可变的二叉树**
```rust
pub trait BinTree {
    type Elem;
    fn as_ref(&self) -> Option<&Self::Elem>;
    fn left(&self) -> Option<&Self::Elem>;
    fn right(&self) -> Option<&Self::Elem>;
    fn move_left(&mut self);
    fn move_right(&mut self);
}
```

**可变的二叉树**
```rust
pub trait BinTreeMut: BinTree {
    fn as_mut(&mut self) -> Option<&mut Self::Elem>;
    fn left_mut(&mut self) -> Option<&mut Self::Elem>;
    fn right_mut(&mut self) -> Option<&mut Self::Elem>;
    fn insert_as_left(&mut self, elem: Self::Elem) -> Option<Self::Elem>;
    fn insert_as_right(&mut self, elem: Self::Elem) -> Option<Self::Elem>;
    fn take_left(&mut self) -> Option<Self>;
    fn take_right(&mut self) -> Option<Self>;
    fn into_inner(self) -> Option<Self::Elem>;
}
```

## 例子与验证

### 顺序二叉树：`VecBinaryTree` struct

// TODO

### 二叉树测试

// TODO

# 文档级别描述

// TODO

# 缺点

// TODO

# 待解决的问题

// TODO

# 其它替代方案

// TODO
