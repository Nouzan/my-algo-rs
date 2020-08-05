/// 计算第n个斐波那契数(递归版本)
pub fn fib_recurrence(n: u64) -> u64 {
    if n == 0 || n == 1 {
        1
    } else {
        fib_recurrence(n - 1) + fib_recurrence(n - 2)
    }
}

/// 计算第n个斐波那契数(非递归版本)
pub fn fib(n: u64) -> u64 {
    let (mut front, mut back) = (1, 1);
    for _ in 2..=n {
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
        let results = vec![1, 1, 2, 3, 5, 8, 13];
        for (idx, v) in results.iter().enumerate() {
            assert_eq!(fib_recurrence(idx as u64), *v);
        }
    }

    #[test]
    fn test_fib() {
        let results = vec![1, 1, 2, 3, 5, 8, 13];
        for (idx, v) in results.iter().enumerate() {
            assert_eq!(fib(idx as u64), *v);
        }
    }
}
