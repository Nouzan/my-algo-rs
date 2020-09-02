use super::LinkedList;

// 把链表分解为奇链和偶链.(分别包含原来链表中的奇数位置结点和偶数位置结点)
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

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_split_odd(data: Vec<i64>) {
            let mut list = LinkedList::from(data.clone());
            let odd = split_odd(&mut list);
            let mut even_cursor = list.cursor();
            let mut odd_cursor = odd.cursor();
            for (idx, elem) in data.iter().enumerate() {
                if idx % 2 == 0 {
                    assert_eq!(odd_cursor.peek(), Some(elem));
                    odd_cursor.move_next();
                } else {
                    assert_eq!(even_cursor.peek(), Some(elem));
                    even_cursor.move_next();
                }
            }
        }
    }
}
