//! 实现了栈与队列相关算法.
//!
//! - 定义了具有`FILO`性质的栈特质: `Stack`.
//! - 定义了具有`FIFO`性质的队特质: `Queue`.

pub mod queue;
pub mod stack;

pub use queue::*;
pub use stack::*;
