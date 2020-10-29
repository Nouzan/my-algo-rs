# 数据结构与算法的Rust实现: my-algo-rs
该项目的目的是在Rust的语境下探讨数据结构与算法的最佳实践，对比我们的实现与标准实现的异同，有助于更好地理解为何它们会被实现成这样.
项目的推进动力来源于相关的数据结构与算法方面的杂题，并无明确的路线.

## 如何研究本项目
克隆本仓库：
```bash
git clone https://github.com/Nouzan/my-algo-rs.git
cd my-algo-rs
```

### 1. 探索文档
- 通过`cargo doc`生成文档
- 在浏览器打开`my-algo-rs/target/doc/my-algo/index.html`.
- 开始你的旅程.

### 2. 直接阅读源码
从`examples`和`test`入手了解本项目.

### 3. 在你自己的项目中进行尝试
创建新的二进制项目:
```
cargo new your-project
```
在`cargo.toml`中添加对该项目的依赖:
```
[dependencies]
my-algo = { git = "https://github.com/Nouzan/my-algo-rs.git" }
```

在项目中引入一些结构，并尝试使用它们:
```rust
use my_algo::{
    ch2::{IndexError, List, ListExt},
    vec::MyVec,
};

fn main() -> Result<(), IndexError> {
    let mut list = MyVec::new();
    for i in 0..10 {
        list.insert(list.len(), i)?;
    }

    list.shift(5)?;

    for i in 0..list.len() {
        println!("{}", list.get(i)?);
    }

    Ok(())
}

```

## 内容

### 向量表(`vec`)
- 参考[The Rustonomicon](https://doc.rust-lang.org/stable/nomicon/)实现的向量: `vec::MyVec`.

### 幂与斐波那契数列(`ch1`)
- 任何幺半群均可使用的幂算法: `ch1::power`.
- 一些斐波那契数列算法: `ch1::{fib, fib_linear, fib_recurrence}`, 其中`ch1::fib`是基于幂算法实现的, 复杂度为`O(lgn)`.

### 线性表(`ch2`)
- 顺序表, 偏等顺序表, 偏序顺序表上的一些算法: `ch2::{List, ListExt, PartialEqListExt, PartialOrdListExt}`, 为`vec::MyVec`和`Vec`实现了上述`Trait`.
- 整数顺序表上的一些算法: `ch2::ISizeListExt`.
- 不带头结点的单链表和带头结点的单链表的实现及其上的一些算法: `ch2::linked_list::{sll::LinkedList, shll::LinkedList}`.
- 不带头结点的循环双链表的实现及其上的一些算法: `ch2::linked_list::cdll::LinkedList`.
- 单链表特质：`ch2::linked_list::{SinglyLinkedList, SinglyLinkedListExt}`.

### 栈与队列(`ch3`)
- 栈特质: `ch3::{Stack, StackExt}`.
- 队列特质: `ch3::{Queue, QueueExt}`.
- 共享栈、双栈: `ch3::{SharedStack, DoubleStack}`, 并用共享栈、双栈实现`Queue`(TODO: 有一些情况下判满算法会出错, 后面发一个issue描述一下情况).
- 循环队列: `ch3::CircularQueue`.

### 树(`ch4`)
- 基本树特质: `BinTree, BinTreeMut, BinTreeCursor, BinTreeCursorMut`
- 可向父母移动的树特质: `MoveParentBinTree, MoveParentBinTreeMut, MoveParentCursor, MoveParentCursorMut`
- 向量二叉树(没有处理空间释放问题，空间性能较差): `vec_binary_tree::VecBinaryTree`
- 链式二叉树(带哨兵根结点): `linked_binary_tree::LinkedBinaryTree`
- 带父母指针的链式二叉树: `doubly_linked_binary_tree::DoublyLinkedBinaryTree`
- 完全二叉堆: `compelete_heap::CompeleteMaxHeap`
- 左式堆: `left_heap::LeftHeap`
- 非平衡二叉查找树(BST): `bst::TreeMap<Tree>`(对树generic), `bst2::TreeMap`(基于不带哨兵根的链式树)
- AVL树: `avlt::AVLTreeMap<Tree>`(要求`Tree: MoveParentBinTreeMut`)
- 伸展树: `st::SplayTree<Tree>`(要求`Tree: MoveParentBinTreeMut`)
- B树: `bt::BTreeMap`(有待将`VecDeque<T>`优化为`[T]`)
- 红黑树: `rbt::RedBackTreeMap<Tree>`(要求`Tree: MoveParentBinTreeMut`)
- 左倾红黑树: `llrbt::RedBackTreeMap`(基于`bst2::TreeMap`)