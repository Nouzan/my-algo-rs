use super::{IndexError, List, ListExt, PartialEqListExt, PartialOrdListExt};
use crate::vec::MyVec;
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_primary(data: Vec<usize>) {
        let mut list= MyVec::new();
        for v in data {
            list.push(v);
        }
        if let Some(p) = list.primary().copied() {
            let mut count = 0;
            for i in 0..list.len() {
                let x = list.get(i).unwrap();
                if *x == p {
                    count += 1;
                }
            }
            prop_assert!(count > list.len() / 2)
        }
    }
}

proptest! {
    #[test]
    fn test_slices(data: Vec<String>, start: usize, end: usize) {
        let mut x: MyVec<String> = MyVec::new();
        for s in data {
            x.insert(x.len(), s).unwrap();
        }
        if start > end || !x.is_index_read_valid(start) || !x.is_index_insert_valid(end) {
            prop_assert!(x.slice_mut(start, end).is_err());
        } else {
            let mut v = vec![];
            for i in start..end {
                v.push(x.get(i).unwrap().clone())
            }

            let slice = x.slice(start, end).unwrap();
            prop_assert_eq!(slice.len(), v.len());
            for i in 0..v.len() {
                prop_assert_eq!(slice.get(i).unwrap(), v.get(i).unwrap())
            }

            let mut slice = x.slice_mut(start, end).unwrap();
            prop_assert_eq!(slice.len(), v.len());
            for i in 0..v.len() {
                prop_assert_eq!(slice.get(i).unwrap(), v.get(i).unwrap())
            }

            v.insert(0, "Hello".to_string());
            slice.insert(0, "Hello".to_string()).unwrap();
            prop_assert_eq!(slice.len(), v.len());
            for i in 0..v.len() {
                prop_assert_eq!(slice.get(i).unwrap(), v.get(i).unwrap())
            }

            v.delete(0).unwrap();
            slice.delete(0).unwrap();
            prop_assert_eq!(slice.len(), v.len());
            for i in 0..v.len() {
                prop_assert_eq!(slice.get(i).unwrap(), v.get(i).unwrap())
            }
        }
    }
}

#[test]
fn test_insert() -> Result<(), IndexError> {
    let mut x: MyVec<usize> = MyVec::new();
    assert_eq!(List::len(&x), 0);
    assert!(List::is_empty(&x));
    List::insert(&mut x, 0, 11)?;
    assert_eq!(*List::get(&x, 0)?, 11);
    assert_eq!(List::len(&x), 1);
    List::insert(&mut x, 0, 12)?;
    assert_eq!(List::len(&x), 2);
    assert_eq!(*List::get(&x, 1)?, 11);
    assert_eq!(*List::get(&x, 0)?, 12);
    Ok(())
}

#[test]
fn test_delete() -> Result<(), IndexError> {
    let mut x: MyVec<usize> = MyVec::new();
    List::insert(&mut x, 0, 11)?;
    assert_eq!(*List::get(&x, 0)?, 11);
    List::insert(&mut x, 0, 12)?;
    assert_eq!(*List::get(&x, 1)?, 11);
    assert_eq!(*List::get(&x, 0)?, 12);
    List::delete(&mut x, 0)?;
    assert_eq!(List::len(&x), 1);
    assert_eq!(*List::get(&x, 0)?, 11);
    List::delete(&mut x, 0)?;
    assert_eq!(List::len(&x), 0);
    Ok(())
}

#[test]
fn test_locate() -> Result<(), IndexError> {
    let mut x: MyVec<usize> = MyVec::new();
    List::insert(&mut x, 0, 11)?;
    assert_eq!(PartialEqListExt::locate(&x, &11), Some(0));
    Ok(())
}

#[test]
fn test_locate_min() -> Result<(), IndexError> {
    let mut x: MyVec<usize> = MyVec::new();
    List::insert(&mut x, 0, 11)?;
    assert_eq!(x.locate_min(), Some(0));
    List::insert(&mut x, 0, 10)?;
    assert_eq!(x.locate_min(), Some(0));
    List::insert(&mut x, 2, 9)?;
    assert_eq!(x.locate_min(), Some(2));
    Ok(())
}

#[test]
fn test_delete_min() -> Result<(), IndexError> {
    let mut x: MyVec<usize> = MyVec::new();
    List::insert(&mut x, 0, 11)?;
    List::insert(&mut x, 0, 10)?;
    List::insert(&mut x, 2, 9)?;
    assert_eq!(x.delete_min(), Some(9));
    assert_eq!(List::len(&x), 2);
    Ok(())
}

#[test]
fn test_reverse() -> Result<(), IndexError> {
    let mut x: MyVec<usize> = MyVec::new();
    List::insert(&mut x, 0, 11)?;
    List::insert(&mut x, 0, 10)?;
    List::insert(&mut x, 2, 9)?;
    // before: 10, 11, 9
    assert_eq!(*List::get(&x, 0)?, 10);
    assert_eq!(*List::get(&x, 1)?, 11);
    assert_eq!(*List::get(&x, 2)?, 9);
    List::reverse(&mut x);
    // after: 9, 11, 10
    assert_eq!(*List::get(&x, 0)?, 9);
    assert_eq!(*List::get(&x, 1)?, 11);
    assert_eq!(*List::get(&x, 2)?, 10);
    List::insert(&mut x, 0, 12)?;
    // before: 12, 9, 11, 10
    List::reverse(&mut x);
    // after: 10, 11, 9, 12
    assert_eq!(*List::get(&x, 0)?, 10);
    assert_eq!(*List::get(&x, 1)?, 11);
    assert_eq!(*List::get(&x, 2)?, 9);
    assert_eq!(*List::get(&x, 3)?, 12);
    Ok(())
}

proptest! {
    #[test]
    fn test_delete_all(data: Vec<usize>) {
        let mut x: MyVec<usize> = MyVec::new();
        for v in data.iter() {
            let len = x.len();
            List::insert(&mut x, len, *v).unwrap();
        }

        if !x.is_empty() {
            let v = *x.get(0).unwrap();
            x.delete_all(&v);
            for i in 0..x.len() {
                let w = x.get(i).unwrap();
                prop_assert_ne!(v, *w);
            }
        }
    }
}

proptest! {
    #[test]
    fn test_sort(data: Vec<usize>) {
        let mut x: MyVec<usize> = MyVec::new();
        for v in data.iter() {
            let len = x.len();
            List::insert(&mut x, len, *v).unwrap();
        }
        x.sort();
        if !x.is_empty() {
            let mut last = x.get(0).unwrap();
            for i in 1..x.len() {
                let now = x.get(i).unwrap();
                prop_assert!(*last <= *now);
                last = now;
            }
        }
    }
}

#[test]
fn test_delete_between() -> Result<(), IndexError> {
    let mut x: MyVec<usize> = MyVec::new();
    let res = x.delete_between(&1, &2);
    assert_eq!(res, vec![]);
    for v in vec![7, 1, 9, 11, 2, 3, 1, 5, 7, 11, 1, 6].iter() {
        let len = x.len();
        List::insert(&mut x, len, *v)?;
    }
    x.sort();
    let res = x.delete_between(&7, &4);
    assert_eq!(res, vec![]);
    let mut res = x.delete_between(&3, &9);
    res.sort();
    for (idx, v) in vec![5, 6, 7, 7].iter().enumerate() {
        assert_eq!(*List::get(&res, idx)?, *v);
    }
    for (idx, v) in vec![1, 1, 1, 2, 3, 9, 11, 11].iter().enumerate() {
        assert_eq!(*List::get(&x, idx)?, *v);
    }
    let mut res = x.delete_between(&0, &100);
    res.sort();
    assert_eq!(res, vec![1, 1, 1, 2, 3, 9, 11, 11]);
    assert!(x.is_empty());

    Ok(())
}

#[test]
fn test_delete_between_opt() -> Result<(), IndexError> {
    let mut x: MyVec<usize> = MyVec::new();
    let res = x.delete_between_opt(&1, &2, true);
    assert_eq!(res, vec![]);
    for v in vec![7, 1, 9, 11, 2, 3, 1, 5, 7, 11, 1, 6].iter() {
        let len = x.len();
        List::insert(&mut x, len, *v)?;
    }
    x.sort();
    let res = x.delete_between_opt(&7, &4, true);
    assert_eq!(res, vec![]);
    let mut res = x.delete_between_opt(&3, &9, true);
    res.sort();
    for (idx, v) in vec![3, 5, 6, 7, 7, 9].iter().enumerate() {
        assert_eq!(*List::get(&res, idx)?, *v);
    }
    for (idx, v) in vec![1, 1, 1, 2, 11, 11].iter().enumerate() {
        assert_eq!(*List::get(&x, idx)?, *v);
    }
    let mut res = x.delete_between_opt(&0, &11, true);
    res.sort();
    assert_eq!(res, vec![1, 1, 1, 2, 11, 11]);
    assert!(x.is_empty());

    Ok(())
}

#[test]
fn test_delete_between_unsorted_sorted() -> Result<(), IndexError> {
    let mut x: MyVec<usize> = MyVec::new();
    let res = x.delete_between_unsorted(&1, &2);
    assert_eq!(res, vec![]);
    for v in vec![7, 1, 9, 11, 2, 3, 1, 5, 7, 11, 1, 6].iter() {
        let len = x.len();
        List::insert(&mut x, len, *v)?;
    }
    x.sort();
    let res = x.delete_between_unsorted(&7, &4);
    assert_eq!(res, vec![]);
    let mut res = x.delete_between_unsorted(&3, &9);
    res.sort();
    for (idx, v) in vec![3, 5, 6, 7, 7, 9].iter().enumerate() {
        assert_eq!(*List::get(&res, idx)?, *v);
    }
    for (idx, v) in vec![1, 1, 1, 2, 11, 11].iter().enumerate() {
        assert_eq!(*List::get(&x, idx)?, *v);
    }
    let mut res = x.delete_between_unsorted(&0, &11);
    res.sort();
    assert_eq!(res, vec![1, 1, 1, 2, 11, 11]);
    assert!(x.is_empty());

    Ok(())
}

#[test]
fn test_delete_between_unsorted_unsorted() -> Result<(), IndexError> {
    let mut x: MyVec<usize> = MyVec::new();
    for v in vec![7, 1, 9, 11, 2, 3, 1, 5, 7, 11, 1, 6].iter() {
        let len = x.len();
        List::insert(&mut x, len, *v)?;
    }
    let mut res = x.delete_between_unsorted(&3, &9);
    res.sort();
    for (idx, v) in vec![3, 5, 6, 7, 7, 9].iter().enumerate() {
        assert_eq!(*List::get(&res, idx)?, *v);
    }
    for (idx, v) in vec![1, 11, 2, 1, 11, 1].iter().enumerate() {
        assert_eq!(*List::get(&x, idx)?, *v);
    }
    let mut res = x.delete_between_unsorted(&0, &11);
    res.sort();
    assert_eq!(res, vec![1, 1, 1, 2, 11, 11]);
    assert!(x.is_empty());

    Ok(())
}

#[test]
fn test_dedup_sorted() -> Result<(), IndexError> {
    let mut x: MyVec<usize> = MyVec::new();
    for v in vec![7, 1, 9, 11, 2, 3, 1, 5, 7, 11, 1, 6].iter() {
        let len = x.len();
        List::insert(&mut x, len, *v)?;
    }
    x.sort();
    x.dedup_sorted();
    assert_eq!(*x, *vec![1, 2, 3, 5, 6, 7, 9, 11]);
    Ok(())
}

#[test]
fn test_merge() -> Result<(), IndexError> {
    let mut x: MyVec<usize> = MyVec::new();
    for i in &[1, 3, 5, 6, 8, 10] {
        x.push(*i)
    }
    let mut y: MyVec<usize> = MyVec::new();
    for i in &[2, 4, 6, 7, 9, 11] {
        y.push(*i)
    }
    let z = x.merge(y);
    assert_eq!(*z, *vec![1, 2, 3, 4, 5, 6, 6, 7, 8, 9, 10, 11]);
    Ok(())
}

#[test]
fn test_shift() -> Result<(), IndexError> {
    let mut x: MyVec<usize> = MyVec::new();
    for i in &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
        x.push(*i);
    }
    x.shift(4)?;
    assert_eq!(*x, *vec![4, 5, 6, 7, 8, 9, 0, 1, 2, 3]);
    Ok(())
}

#[test]
fn test_search() -> Result<(), IndexError> {
    let mut x: MyVec<usize> = MyVec::new();
    for i in &[1, 2, 3, 3, 3, 4, 5, 6, 9, 11] {
        x.push(*i);
    }
    assert_eq!(x.search(&3), Some(4));
    assert_eq!(x.search(&4), Some(5));
    assert_eq!(x.search(&0), None);
    assert_eq!(x.search(&12), Some(9));
    assert_eq!(x.search(&1), Some(0));
    assert_eq!(x.search(&11), Some(9));
    let mut x: MyVec<usize> = MyVec::new();
    x.push(1);
    assert_eq!(x.search(&1), Some(0));
    assert_eq!(x.search(&0), None);
    assert_eq!(x.search(&11), Some(0));
    Ok(())
}

#[test]
fn test_mid() -> Result<(), IndexError> {
    let mut x: MyVec<usize> = MyVec::new();
    for i in &[11, 13, 15, 17, 19] {
        x.push(*i);
    }
    assert_eq!(x.mid(), Some(&15));

    Ok(())
}

proptest! {
    #[test]
    fn test_merge_mid(a: Vec<isize>, b: Vec<isize>) {
        let mut first: MyVec<isize> = MyVec::new();
        let mut second: MyVec<isize> = MyVec::new();
        let mut merged: MyVec<isize> = MyVec::new();
        for v in a.iter() {
            let len = first.len();
            List::insert(&mut first, len, *v).unwrap();
            let len = merged.len();
            List::insert(&mut merged, len, *v).unwrap();
        }
        for v in b.iter() {
            let len = second.len();
            List::insert(&mut second, len, *v).unwrap();
            let len = merged.len();
            List::insert(&mut merged, len, *v).unwrap();
        }
        first.sort();
        second.sort();
        merged.sort();
        let mid = first.merge_mid(&second).copied();
        let merged_mid = merged.mid().copied();
        assert_eq!(mid, merged_mid);
    }
}

#[test]
fn test_merge_mid_debug() {
    let mut first: MyVec<isize> = MyVec::new();
    let mut second: MyVec<isize> = MyVec::new();
    let mut merged: MyVec<isize> = MyVec::new();
    let a = vec![
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -2897281253,
        -6196200173237906148,
        8925592336651969408,
        2455274111052877208,
        -656826015673857378,
        -3918187044768650842,
        -7166999091199384584,
        -5996361932006152609,
        2889077960617313680,
        88271664092555393,
        2899996647040923106,
        -6516672597673576215,
        -5546300437270337926,
        1473724560447567648,
        2059516156938740199,
        -171858695312451639,
        8522714349128348863,
        4264948759076353221,
        1594454708689104818,
        7681715165400207596,
        -8148621855136367066,
        -644209070338910506,
        5220468932815270648,
        -6625493688883664448,
        1397876726542150767,
        -4727728100751890483,
        7531905508060828888,
        6519918035677179251,
        -5583427334731038398,
        -2967049700757160626,
        -1721576352507059971,
        1727757225621534506,
        2170186811265331760,
        5922347655179323346,
        -8204506787910346843,
        3005523099967943604,
        -1605301380642725877,
        -7332893205306610347,
        -938746561832165523,
        -8789972477573055083,
        -3835164436479195405,
        4786210369317527761,
        3594225956822071679,
        -1748670941390812505,
        3876800443306823381,
        -2257341117945605237,
        5677367518449984234,
        8405782022434682455,
        7581027924183879849,
        4096207420437071452,
        8085920046616710860,
    ];
    let b = vec![
        7482600966255041402,
        -8333722760038280943,
        -4250333620945924187,
        -880643609219279756,
        -5071519582309135839,
        -3223679753249750427,
        99164523170473582,
        3290501520063790669,
        -4598488739711737148,
        4473989299141740021,
        -6781163372128545589,
        5548457122780486112,
        -3557150876905369710,
        -6908408383691144508,
        -6691672864717401851,
        -1937234497355224888,
        1707928323010534440,
        -6339963453647765820,
        7531816131263962515,
        -1284471083586039299,
        7403650438578929422,
        -3829572531986954543,
        386140615396125578,
        -7203738925830004739,
        -8544999182076961763,
        -1490629782192538174,
        5090487921526136898,
        -5141834306885877895,
        -6956565386351062722,
        -7576159871494786891,
        -7491376982597399724,
        4720093450235912204,
        -4053929147728379618,
        4161325017029619931,
        -7081740323715740893,
        8102254179923436400,
        461968019096134908,
        2689246687889717639,
        -7665274172393783307,
        4662732249364662193,
        70100343326846188,
        6099973236709120471,
        -5341597363607795057,
        -7862231724152292154,
        734124934836851694,
        2449474722449367057,
        3081651409021500712,
        7122530452107911687,
        -6074493196840102323,
        4838248576879314072,
        7383191579363050811,
        -3914598055905828817,
        3109065319387327394,
        -1781297907957027685,
        -3583352287771982849,
    ];
    for v in a.iter() {
        let len = first.len();
        List::insert(&mut first, len, *v).unwrap();
        let len = merged.len();
        List::insert(&mut merged, len, *v).unwrap();
    }
    for v in b.iter() {
        let len = second.len();
        List::insert(&mut second, len, *v).unwrap();
        let len = merged.len();
        List::insert(&mut merged, len, *v).unwrap();
    }
    first.sort();
    second.sort();
    merged.sort();
    let mid = first.merge_mid(&second).copied();
    let merged_mid = merged.mid().copied();
    assert_eq!(mid, merged_mid);
}
