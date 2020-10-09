- 特性名称: `binary_tree`
- 提案创建日期: 2020-10-07
- RFC PR: [#0043](https://github.com/Nouzan/my-algo-rs/pull/0043)
- 关联 Issue: [#0044](https://github.com/Nouzan/my-algo-rs/issues/0044)

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

实现的例子请参考**文档级别描述**中的`VecBinaryTree`.

# 文档级别描述

## 实现

特质的实现参考**指南级别描述**.

## 顺序二叉树：`VecBinaryTree` struct

### 顺序二叉树主体

**定义**

*顺序二叉树*实际是一个*顺序表*，因此我们采用`Vec`作为`VecBinaryTree`的底层结构.

我们的实现思路是在底层上将`VecBinaryTree`组织成一棵*完全二叉树*，使得树上每一个合法的位置与顺序表中的下标一一对应，我们把它称之为**位置树**.
而真实的树结构则通过**位置树**上每一个结点是否存在来表示(即**位置树**的内容类型为`Option<T>`，在给定位置上的确有树结点时为`Some(T)`否则为`None`).
于是`VecBinaryTree`结构可以定义为：
```rust
pub struct VecBinaryTree<T> {
    inner: Vec<Option<T>>,
}
```

**工具函数**

于是，树根位置在`inner`中的下标为`0`，对于每一个结点位置(下标记为`x`)，它的左孩子位置下标为`2*x+1`，右孩子位置下标为`2*x+2`. 我们定义了两个工具函数来计算给定下标结点的左孩子下标和右孩子下标：
```rust
pub(super) const fn left_index(index: usize) -> usize {
    2 * index + 1
}

pub(super) const fn right_index(index: usize) -> usize {
    2 * index + 2
}
```

最后，为了访问给定位置的结点，我们定义了两个工具方法：
```rust
impl<T> VecBinaryTree<T> {
    fn get(&self, index: usize) -> Option<&T> {
        self.inner.get(index).map_or(None, |elem| elem.as_ref())
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.inner.get_mut(index).map_or(None, |elem| elem.as_mut())
    }

    // ...
}
```
若给定位置还不存在(即`inner`还未对该位置进行初始化)或位置存在但该位置上不存在结点(即值为`None`)，则都返回`None`；否则用`Some`包裹着该位置结点内容的引用并返回.

为了实现`BinTree`和`BinTreeMut`特质，我们还需要为`VecBinaryTree`定义相应的游标类型.

### 游标

**定义**

实际上，**位置树**上的一个位置可以由它的下标来唯一确定，因此游标直接定义为下标再加上对树的相应级别的引用即可. 下面的定义直接反映了这一点：
```rust
pub struct Cursor<'a, T> {
    pub(super) current: usize,
    pub(super) tree: &'a VecBinaryTree<T>,
}

pub struct CursorMut<'a, T> {
    pub(super) current: usize,
    pub(super) tree: &'a mut VecBinaryTree<T>,
}
```

`Cursor`用于定义`VecBinaryTree::Node`(为`Cursor`实现`Clone`是容易的)，`CursorMut`用于定义`VecBinaryTree::NodeMut`.

**实现`BinTreeNode`**

则内容访问和位置的相对移动的实现可以简单地使用`VecBinaryTree`中定义的工具函数来实现. 下面仅给出`Cursor`的例子：
```rust
impl<'a, T> BinTreeNode for Cursor<'a, T> {
    fn as_ref(&self) -> Option<&Self::Elem> {
        self.tree.get(self.current)
    }

    fn left(&self) -> Option<&Self::Elem> {
        self.tree.get(left_index(self.current))
    }

    fn right(&self) -> Option<&Self::Elem> {
        self.tree.get(right_index(self.current))
    }

    fn move_left(&mut self) {
        if !self.is_empty() {
            let idx = left_index(self.current);
            self.current = idx;
        }
    }

    fn move_right(&mut self) {
        if !self.is_empty() {
            let idx = right_index(self.current);
            self.current = idx;
        }
    }
}
```

**实现`BinTree`和`BinTreeMut`**

`BinTreeNode`和`BinTreeNodeMut`要求分别递归实现`BinTree`和`BinTreeMut`，我们只需要简单地将关联类型定义为游标自身，并且游标创建方法定义为自身的复制即可. 需要注意的是，我们要将`CursorMut::Node`定义为`Cursor`，以满足`Node`对`Clone`的要求.

**实现`BinTreeNodeMut`**

对于关联元素的插入，我们首先需要确保`inner`已经完成对关联位置的初始化，对于一个已初始化好的位置，我们直接将它的值修改为要插入的元素即可. 这里我们使用`Vec::resize_with`方法来进行初始化. 下面是`insert_as_left`的例子：
```rust
impl<'a, T> BinTreeNodeMut for CursorMut<'a, T> {
    // ...
    fn insert_as_left(&mut self, elem: Self::Elem) -> Option<Self::Elem> {
        if self.left().is_none() && !self.is_empty() {
            let idx = self.get_left_index_and_resize();
            let left = self.tree.inner.get_mut(idx).unwrap();
            *left = Some(elem);
            None
        } else {
            Some(elem)
        }
    }
    // ...
}
```

其中`CursorMut::get_left_index_and_resize`方法在获取左孩子位置的同时确保到左孩子位置为止的位置都经过了初始化：
```rust
impl<'a, T> CursorMut<'a, T> {
    fn get_left_index_and_resize(&mut self) -> usize {
        let idx = left_index(self.current);
        if idx >= self.tree.inner.len() {
            self.tree.inner.resize_with(idx + 1, || None);
        }
        idx
    }
    // ...
}
```

为了实现*结构操作*(如`take_left`和`append_left`)，我们必须理解**位置树**的子树是如何存储在`Vec`中的.
事实上我们有如下结果：

- 设一结点在原树中的下标为`x in 0..`，则以它为根的子树中每个结点，若它在子树中的相对下标为`n in 0..`，`n + 1 == 2.pow(m) + l` 且 `l in 0..(2.pow(m))`，则它在原树中的下标为`n + x * 2.pow(m)` (实际上，`2.pow(m)`即为不超过`n+1`的最大的`2`的幂). 我们把这个公式称为**下标公式**.

因此，要遍历一棵**位置子树**，我们只需要根据上述结果，直接给出每个**位置结点**的下标，然后在`Vec`中访问它即可. 下面给出了`CursorMut::take_left`的实现：
```rust
impl<'a, T> BinTreeNodeMut for CursorMut<'a, T> {
    // ...
    fn take_left(&mut self) -> Option<Self::Tree>
        where
            Self::Tree: Sized,
        {
            if self.is_empty() {
                None
            } else {
                let mut tree = VecBinaryTree::new();
                let mut cursor = tree.cursor_mut();
                let iter = InOrderIndexIter::new(left_index(self.current), self.tree.inner.len());
                for (dst, src) in iter.enumerate() {
                    if src < self.tree.inner.len() {
                        let src_node = self.get_node_and_resize(src);
                        let dst_node = cursor.get_node_and_resize(dst);
                        *dst_node = src_node.take();
                    } else {
                        break;
                    }
                }
                Some(tree)
            }
        }
    // ...
}
```
其中`InOrderIndexIter`是根据**下标公式**实现的迭代器. `CursorMut::get_node_and_resize`是确保给定下标以被初始化并返回结点的可变引用的工具方法，它的实现为：
```rust
impl<'a, T> CursorMut<'a, T> {
    fn get_node_and_resize(&mut self, index: usize) -> &mut Option<T> {
        if index >= self.tree.inner.len() {
            self.tree.inner.resize_with(index + 1, || None);
        }
        self.tree.inner.get_mut(index).unwrap()
    }
}
```

最后，我们便可以使用`Cursor`和`CursorMut`为`VecBinaryTree`实现`BinTree`和`BinTreeMut`了：
```rust
impl<T> BinTree for VecBinaryTree<T> {
    type Elem = T;
    type Node<'a, E: 'a> = Cursor<'a, E>;

    fn cursor<'a>(&'a self) -> Self::Node<'a, Self::Elem> {
        Cursor {
            current: 0,
            tree: self,
        }
    }
}

impl<T> BinTreeMut for VecBinaryTree<T> {
    type NodeMut<'a, E: 'a> = CursorMut<'a, E>;

    fn cursor_mut<'a>(&'a mut self) -> Self::NodeMut<'a, Self::Elem> {
        CursorMut {
            current: 0,
            tree: self,
        }
    }
}
```

## 测试二叉树

利用通用的前序、中序、后序以及层序遍历算法，然后用待测试的二叉树结构去实现一些标准二叉树样例，最后只需检验每种遍历结果都与标准遍历结果一致即可.

# 缺点

- 游标把*位置*抽象与*内容访问权*抽象紧密地耦合在了一起，使得我们不容易使用游标去实现一些*保结构的算法*，比如实现`map`方法.
- 实现中使用了`#![feature(generic_associated_types)]`它是一个未完成的feature([rust-lang/rust#44265](https://github.com/rust-lang/rust/issues/44265)).

# 待解决的问题

- 为`BinTreeNode`和`BinTreeNodeMut`递归地实现`BinTree`和`BinTreeMut`.

# 替代方案

征集中...