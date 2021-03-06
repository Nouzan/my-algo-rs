use my_algo::{vec::MyVec, IndexError, List, ListExt};

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
