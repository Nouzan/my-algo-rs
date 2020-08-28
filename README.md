# my-algo-rs: 数据结构与算法的Rust实现
该项目的目的是在Rust的语境下探讨数据结构与算法的最佳实践，对比我们的实现与标准实现的异同，有助于更好地理解为何它们会被实现成这样.
项目的推进动力来源于相关的数据结构与算法方面的杂题，并无明确的路线.

## 内容

### 向量表(`vec`)
- 参考[The Rustonomicon](https://doc.rust-lang.org/stable/nomicon/)实现的向量: `vec::MyVec`.

### 幂与斐波那契数列(`ch1`)
- 任何幺半群均可使用的幂算法: `ch1::power`.
- 一些斐波那契数列算法: `ch1::{fib, fib_linear, fib_recurrence}`, 其中`ch1::fib`是基于幂算法实现的, 复杂度为`O(lgn)`.

### 线性表(`ch2`)
- 顺序表, 偏等顺序表, 偏序顺序表上的一些算法: `ch2::{List, ListExt, PartialEqListExt, PartialOrdListExt}`, 为`vec::MyVec`和`Vec`实现了上述`Trait`.
- 整数顺序表上的一些算法: `ch2::ISizeListExt`.
- 不带头结点的单链表和带头结点的单链表的实现及其上的一些算法: `ch2::LinkedList::{single, single_head}`.
