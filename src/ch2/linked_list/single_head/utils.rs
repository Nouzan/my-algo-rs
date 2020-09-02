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

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    proptest! {
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
