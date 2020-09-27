//! 实现了栈与队列相关算法.
//!
//! - 定义了具有`LIFO`性质的栈特质: `Stack`.
//! - 定义了具有`FIFO`性质的队特质: `Queue`.

mod operators;
pub mod queue;
pub mod stack;

pub use queue::*;
pub use stack::*;

use thiserror::Error;

/// `ch3`中的错误类型.
#[derive(Error, Debug)]
pub enum Error {
    /// 栈上溢.
    #[error("stack overflow.")]
    StackOverflow,

    /// 表达式不合法.
    #[error("Not a valid expression.")]
    ExpressionNotValid,
}
