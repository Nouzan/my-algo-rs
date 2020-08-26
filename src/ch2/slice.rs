use super::{IndexError, List};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Slice<'a, L, Item> {
    list: &'a L,
    start: usize,
    len: usize,
    _marker: PhantomData<Item>,
}

impl<'a, L, Item> Slice<'a, L, Item>
where
    L: List<Item>,
{
    pub fn new(list: &'a L, start: usize, len: usize) -> Result<Self, IndexError> {
        if list.is_index_read_valid(start) && list.is_index_insert_valid(start + len) {
            Ok(Self {
                list,
                start,
                len,
                _marker: PhantomData::default(),
            })
        } else {
            Err(IndexError)
        }
    }

    fn index(&self, i: usize) -> Result<usize, IndexError> {
        if self.is_index_read_valid(i) {
            Ok(i + self.start)
        } else {
            Err(IndexError)
        }
    }
}

impl<'a, L, Item> List<Item> for Slice<'a, L, Item>
where
    L: List<Item>,
{
    fn len(&self) -> usize {
        self.len
    }

    fn get(&self, index: usize) -> Result<&Item, IndexError> {
        self.list.get(self.index(index)?)
    }

    fn get_mut(&mut self, _index: usize) -> Result<&mut Item, IndexError> {
        Err(IndexError)
    }

    fn swap(&mut self, _i: usize, _j: usize) -> Result<(), IndexError> {
        Err(IndexError)
    }

    fn insert(&mut self, _index: usize, _x: Item) -> Result<(), IndexError> {
        Err(IndexError)
    }

    fn delete(&mut self, _index: usize) -> Result<Item, IndexError> {
        Err(IndexError)
    }
}

#[derive(Debug)]
pub struct SliceMut<'a, L, Item> {
    list: &'a mut L,
    start: usize,
    len: usize,
    _marker: PhantomData<Item>,
}

impl<'a, L, Item> SliceMut<'a, L, Item>
where
    L: List<Item>,
{
    pub fn new(list: &'a mut L, start: usize, len: usize) -> Result<Self, IndexError> {
        if list.is_index_read_valid(start) && list.is_index_insert_valid(start + len) {
            Ok(Self {
                list,
                start,
                len,
                _marker: PhantomData::default(),
            })
        } else {
            Err(IndexError)
        }
    }

    fn index(&self, i: usize) -> Result<usize, IndexError> {
        if self.is_index_read_valid(i) {
            Ok(i + self.start)
        } else {
            Err(IndexError)
        }
    }

    fn insert_index(&self, i: usize) -> Result<usize, IndexError> {
        if self.is_index_insert_valid(i) {
            Ok(i + self.start)
        } else {
            Err(IndexError)
        }
    }
}

impl<'a, L, Item> List<Item> for SliceMut<'a, L, Item>
where
    L: List<Item>,
{
    fn len(&self) -> usize {
        self.len
    }

    fn get(&self, index: usize) -> Result<&Item, IndexError> {
        self.list.get(self.index(index)?)
    }

    fn get_mut(&mut self, index: usize) -> Result<&mut Item, IndexError> {
        self.list.get_mut(self.index(index)?)
    }

    fn swap(&mut self, i: usize, j: usize) -> Result<(), IndexError> {
        let i = self.index(i)?;
        let j = self.index(j)?;
        self.list.swap(i, j)
    }

    fn insert(&mut self, index: usize, x: Item) -> Result<(), IndexError> {
        let index = self.insert_index(index)?;
        if self.list.insert(index, x).is_ok() {
            self.len += 1;
            Ok(())
        } else {
            Err(IndexError)
        }
    }

    fn delete(&mut self, index: usize) -> Result<Item, IndexError> {
        let index = self.index(index)?;
        if let Ok(item) = self.list.delete(index) {
            self.len -= 1;
            Ok(item)
        } else {
            Err(IndexError)
        }
    }
}
