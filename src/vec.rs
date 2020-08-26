use crate::ch2::{IndexError, List};
use std::alloc::{alloc, dealloc, realloc, Layout};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::{self, NonNull};

#[derive(Debug)]
struct RawVec<T> {
    ptr: NonNull<T>,
    cap: usize,
    _marker: PhantomData<T>, // for drop checker.
}

impl<T> RawVec<T> {
    fn new() -> Self {
        let layout = Layout::new::<T>();
        let cap = if layout.size() == 0 { !0 } else { 0 }; // for ZSTs, `cap` will always be `usize::MAX`(`!0`).
        Self {
            ptr: NonNull::dangling(),
            cap,
            _marker: PhantomData::default(),
        }
    }

    fn grow(&mut self) {
        unsafe {
            let layout = Layout::new::<T>();
            assert!(layout.size() != 0, "capacity overflow");
            let (new_cap, ptr) = if self.cap == 0 {
                let ptr = alloc(layout);
                (1, ptr)
            } else {
                let new_cap = self.cap * 2;
                let old_num_bytes = self.cap * layout.size();
                assert!(
                    old_num_bytes <= (isize::MAX as usize / 2),
                    "capacity overflow"
                );
                let new_num_bytes = old_num_bytes * 2;
                let ptr = realloc(self.ptr.as_ptr() as *mut _, layout, new_num_bytes);
                (new_cap, ptr)
            };

            self.ptr = NonNull::new(ptr as *mut _).expect("run out of memory!");
            self.cap = new_cap;
        }
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        let layout = Layout::new::<T>();
        if self.cap != 0 && layout.size() != 0 {
            unsafe {
                dealloc(self.ptr.as_ptr() as *mut _, layout);
            }
        }
    }
}

/// 自定义的`Vec`, 采用和`Vec`一样的翻倍扩容策略.
#[derive(Debug)]
pub struct MyVec<T> {
    buf: RawVec<T>,
    len: usize,
}

impl<T> MyVec<T> {
    fn ptr(&self) -> *mut T {
        self.buf.ptr.as_ptr()
    }

    fn cap(&self) -> usize {
        self.buf.cap
    }

    /// 创建一个新的`MyVec`.
    pub fn new() -> Self {
        Self {
            buf: RawVec::new(),
            len: 0,
        }
    }

    /// 在`MyVec`的末尾插入一个新值.
    pub fn push(&mut self, elem: T) {
        if self.len == self.cap() {
            self.buf.grow();
        }

        unsafe {
            ptr::write(self.ptr().add(self.len), elem);
        }

        self.len += 1;
    }

    /// 弹出`MyVec`末尾的值, 若表空则返回`None`.
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.ptr().add(self.len))) }
        }
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

impl<T> Deref for MyVec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr(), self.len) }
    }
}

impl<T> DerefMut for MyVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.ptr(), self.len) }
    }
}

impl<T> Default for MyVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> List<T> for MyVec<T> {
    fn len(&self) -> usize {
        self.len
    }

    fn get(&self, index: usize) -> Result<&T, IndexError> {
        if self.is_index_read_valid(index) {
            unsafe { Ok(self.ptr().add(index).as_ref().unwrap()) }
        } else {
            Err(IndexError)
        }
    }

    fn get_mut(&mut self, index: usize) -> Result<&mut T, IndexError> {
        if self.is_index_read_valid(index) {
            unsafe { Ok(self.ptr().add(index).as_mut().unwrap()) }
        } else {
            Err(IndexError)
        }
    }

    fn swap(&mut self, i: usize, j: usize) -> Result<(), IndexError> {
        if self.is_index_read_valid(i) && self.is_index_read_valid(j) {
            unsafe {
                ptr::swap(self.ptr().add(i), self.ptr().add(j));
                Ok(())
            }
        } else {
            Err(IndexError)
        }
    }

    fn insert(&mut self, index: usize, x: T) -> Result<(), IndexError> {
        if self.is_index_insert_valid(index) {
            if self.len == self.cap() {
                self.buf.grow();
            }
            unsafe {
                if self.is_index_read_valid(index) {
                    ptr::copy(
                        self.ptr().add(index),
                        self.ptr().add(index + 1),
                        self.len - index,
                    );
                }
                ptr::write(self.ptr().add(index), x);
                self.len += 1;
                Ok(())
            }
        } else {
            Err(IndexError)
        }
    }

    fn delete(&mut self, index: usize) -> Result<T, IndexError> {
        if self.is_index_read_valid(index) {
            unsafe {
                // if index is read valid, then must we have 0 <= index < self.len,
                // which means self.len > 0.
                self.len -= 1;
                let res = ptr::read(self.ptr().add(index));
                ptr::copy(
                    self.ptr().add(index + 1),
                    self.ptr().add(index),
                    self.len - index,
                );
                Ok(res)
            }
        } else {
            Err(IndexError)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct Foo(usize);

    struct FooZst;

    impl Drop for Foo {
        fn drop(&mut self) {
            println!("drop: {}", self.0);
        }
    }

    #[test]
    fn test_basic() {
        let mut v: MyVec<&'static str> = MyVec::new();
        v.push("Hello");
        v.push("World");
        assert_eq!(v.pop(), Some("World"));
        assert_eq!(v.pop(), Some("Hello"));
        assert_eq!(v.pop(), None);
    }

    #[test]
    fn test_drop() {
        {
            let mut v = MyVec::new();
            v.push(Foo(1));
            v.push(Foo(2));
            v.push(Foo(3));
        }
        {
            let mut v = MyVec::new();
            v.push(Foo(1));
            v.push(Foo(2));
            v.push(Foo(3));
        }
    }

    #[test]
    fn test_deref() {
        let mut v = MyVec::new();
        for i in 0..10 {
            v.push(i);
        }
        let mapped: Vec<_> = v.iter_mut().map(|v| *v * 2).collect();
        assert_eq!(mapped, vec![0, 2, 4, 6, 8, 10, 12, 14, 16, 18]);
    }

    #[test]
    fn test_zst() {
        let mut v = MyVec::new();
        for _ in 0..100 {
            v.push(FooZst);
        }
        for _ in 0..100 {
            assert!(v.pop().is_some());
        }
    }
}
