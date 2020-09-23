//! 一些简单算法: 斐波那契数列算法、幂算法.

use num_traits::One;
use std::ops::Mul;

/// `2x2`矩阵.
#[derive(Debug, Clone)]
struct Mat(u128, u128, u128, u128);

impl Mul for Mat {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let Self(a00, a01, a10, a11) = self;
        let Self(b00, b01, b10, b11) = rhs;
        Self(
            a00 * b00 + a01 * b10,
            a00 * b01 + a01 * b11,
            a10 * b00 + a11 * b10,
            a10 * b01 + a11 * b11,
        )
    }
}

impl One for Mat {
    fn one() -> Self {
        Self(1, 0, 0, 1)
    }
}

/// 幂算法. 通过平方求幂.
pub fn power<T: Mul + One + Clone>(mut x: T, mut n: usize) -> T {
    if n == 0 {
        T::one()
    } else {
        while n & 1 == 0 {
            x = x.clone() * x;
            n >>= 1;
        }
        let mut acc = x.clone();
        while n > 1 {
            x = x.clone() * x;
            n >>= 1;
            if n & 1 == 1 {
                acc = acc.clone() * x.clone();
            }
        }
        acc
    }
}

/// 计算第n个斐波那契数(`O(lgn)`版本)
pub fn fib(n: usize) -> u128 {
    let mat = power(Mat(0, 1, 1, 1), n);
    mat.1
}

/// 计算第n个斐波那契数(递归版本)
pub fn fib_recurrence(n: usize) -> u128 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib_recurrence(n - 1) + fib_recurrence(n - 2)
    }
}

/// 计算第n个斐波那契数(非递归版本)
pub fn fib_linear(n: usize) -> u128 {
    let (mut front, mut back) = (0, 1);
    for _ in 1..=n {
        let new = front + back;
        back = front;
        front = new;
    }
    front
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fib_recurrence() {
        let results = vec![0, 1, 1, 2, 3, 5, 8, 13];
        for (idx, v) in results.iter().enumerate() {
            assert_eq!(fib_recurrence(idx), *v);
        }
    }

    #[test]
    fn test_fib_linear() {
        let results = vec![0, 1, 1, 2, 3, 5, 8, 13];
        for (idx, v) in results.iter().enumerate() {
            assert_eq!(fib_linear(idx), *v);
        }
    }

    #[test]
    fn test_fib() {
        let results = vec![0, 1, 1, 2, 3, 5, 8, 13];
        for (idx, v) in results.iter().enumerate() {
            assert_eq!(fib(idx), *v);
        }
    }

    #[test]
    fn test_fib_185() {
        assert_eq!(fib(185), fib_linear(185));
    }

    #[test]
    fn test_power() {
        assert_eq!(power(11, 21), (11u128).pow(21));
    }
}
