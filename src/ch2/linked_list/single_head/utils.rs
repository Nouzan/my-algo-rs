use super::LinkedList;

/// 把链表分解为奇链和偶链.(分别包含原来链表中的奇数位置结点和偶数位置结点)
// 习题 2.3.10
pub fn split_odd<T>(list: &mut LinkedList<T>) -> LinkedList<T> {
    let mut cursor = list.cursor_mut();
    let mut odd = LinkedList::default();
    let mut odd_cursor = odd.cursor_mut();
    let mut idx = 0;
    while cursor.peek().is_some() {
        idx += 1;
        if idx % 2 == 1 {
            odd_cursor.insert_after(cursor.remove_current().unwrap());
            odd_cursor.move_next();
        } else {
            cursor.move_next();
        }
    }
    odd
}

/// 把两个递增排列的有序单链表合并为一个递增排列的有序单链表.
/// # Correctness
/// 要求`lhs`和`rhs`递增有序.
// 习题 2.3.13
pub fn merge<T: PartialOrd>(mut lhs: LinkedList<T>, mut rhs: LinkedList<T>) -> LinkedList<T> {
    let mut merged = LinkedList::default();
    let mut cursor = merged.cursor_mut();
    let mut lcr = lhs.cursor_mut();
    let mut rcr = rhs.cursor_mut();
    while let (Some(le), Some(re)) = (lcr.as_cursor().peek(), rcr.as_cursor().peek()) {
        if *le < *re {
            cursor.insert_after(lcr.remove_current().unwrap());
        } else {
            cursor.insert_after(rcr.remove_current().unwrap());
        }
        cursor.move_next();
    }
    merged.append(lhs);
    merged.append(rhs);
    merged
}

/// 生成一个单链表, 它包含两个递增有序的单链表的公共元素.
/// # Correctness
/// 要求`lhs`和`rhs`递增有序.
/// # Examples
/// ```
/// use my_algo::ch2::linked_list::single_head::{LinkedList, utils::common};
///
/// let lhs = LinkedList::from(vec![1, 2, 2, 3, 4]);
/// let rhs = LinkedList::from(vec![1, 2, 2, 3, 3, 4]);
/// assert_eq!(common(&lhs, &rhs), vec![1, 2, 2, 3, 4]);
/// ```
// 习题 2.3.14
pub fn common<T: PartialOrd + Clone>(lhs: &LinkedList<T>, rhs: &LinkedList<T>) -> LinkedList<T> {
    let mut common = LinkedList::default();
    let mut curosr = common.cursor_mut();
    let mut lcur = lhs.cursor();
    let mut rcur = rhs.cursor();
    while let (Some(le), Some(re)) = (lcur.peek(), rcur.peek()) {
        if *le == *re {
            curosr.insert_after(le.clone());
            curosr.move_next();
            lcur.move_next();
            rcur.move_next();
        } else if *le < *re {
            lcur.move_next();
        } else {
            rcur.move_next();
        }
    }
    common
}

/// 求两个链表元素集合的交集.
/// # Correctness
/// 要求`lhs`和`rhs`递增有序.
/// # Examples
/// ```
/// use my_algo::ch2::linked_list::single_head::{LinkedList, utils::intersect};
///
/// let mut lhs = LinkedList::from(vec![1, 2, 2, 3, 4]);
/// let rhs = LinkedList::from(vec![1, 2, 2, 3, 3, 4]);
/// intersect(&mut lhs, &rhs);
/// assert_eq!(lhs, vec![1, 2, 3, 4]);
/// ```
// 习题 2.3.15
pub fn intersect<T: PartialOrd>(lhs: &mut LinkedList<T>, rhs: &LinkedList<T>) {
    let mut lcur = lhs.cursor_mut();
    let mut rcur = rhs.cursor();
    while let (Some(le), Some(re)) = (lcur.as_cursor().peek(), rcur.peek()) {
        if *le == *re {
            lcur.move_next();
            while lcur.as_cursor().peek() == Some(re) {
                lcur.remove_current();
            }
            rcur.move_next();
        } else if *le < *re {
            lcur.remove_current();
        } else {
            rcur.move_next();
        }
    }
    while lcur.as_cursor().peek().is_some() {
        lcur.remove_current();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_intersect(data1: Vec<i64>, data2: Vec<i64>) {
            let mut lhs = LinkedList::from(data1);
            let mut rhs = LinkedList::from(data2);
            lhs.sort();
            rhs.sort();
            let mut common = common(&lhs, &rhs);
            common.dedup();
            intersect(&mut lhs, &rhs);
            prop_assert_eq!(lhs, Vec::from(common));
        }

        #[test]
        fn test_common(mut data1: Vec<i64>, mut data2: Vec<i64>) {
            let mut lhs = LinkedList::from(data1.clone());
            let mut rhs = LinkedList::from(data2.clone());
            lhs.sort();
            rhs.sort();
            let common = common(&lhs, &rhs);
            let mut cursor = common.cursor();
            let mut j = 0;
            data1.sort_unstable();
            data2.sort_unstable();
            for item in data1.iter().take(data1.len().min(data2.len())) {
                match item.cmp(&data2[j]) {
                    std::cmp::Ordering::Equal => {
                        prop_assert_eq!(cursor.peek(), Some(item));
                        j += 1;
                        cursor.move_next();
                    },
                    std::cmp::Ordering::Greater => {
                        j += 1;
                    }
                    _ => (),
                }
            }
            prop_assert!(cursor.peek().is_none());
        }

        #[test]
        fn test_merge(mut data1: Vec<i64>, mut data2: Vec<i64>) {
            let mut lhs = LinkedList::from(data1.clone());
            let mut rhs = LinkedList::from(data2.clone());
            lhs.sort();
            rhs.sort();
            data1.append(&mut data2);
            data1.sort();
            let merged = merge(lhs, rhs);
            prop_assert_eq!(data1, merged);
        }

        #[test]
        fn test_split_odd(data: Vec<i64>) {
            let mut list = LinkedList::from(data.clone());
            let odd = split_odd(&mut list);
            let mut even_cursor = list.cursor();
            let mut odd_cursor = odd.cursor();
            for (idx, elem) in data.iter().enumerate() {
                if idx % 2 == 0 {
                    prop_assert_eq!(odd_cursor.peek(), Some(elem));
                    odd_cursor.move_next();
                } else {
                    prop_assert_eq!(even_cursor.peek(), Some(elem));
                    even_cursor.move_next();
                }
            }
        }
    }
}
