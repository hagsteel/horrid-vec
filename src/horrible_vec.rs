use std::alloc::{alloc, dealloc, Layout};
use std::fmt;
use std::mem::{align_of, size_of};
use std::ops::Index;
use std::ptr;

#[derive(Debug)]
pub struct HorridVec<T: fmt::Debug> {
    inner: *mut T, // to get T at an index: get inner.offset(index).read() -> T
    offset: isize,
    len: usize,
    capacity: usize,
}

// Capacity is the entire memory space
// wheras the alignment is the size of a slot for T
// `value_1` can span slot 1 and 2 so memory
// S1----S2----S3
// [value_1]
impl<T: fmt::Debug> HorridVec<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        unsafe {
            let layout =
                Layout::from_size_align(capacity * size_of::<T>(), align_of::<T>()).unwrap();
            let mem = alloc(layout);
            let start = mem.cast::<T>();

            Self {
                inner: start,
                len: 0,
                offset: 0,
                capacity,
            }
        }
    }

    pub fn push(&mut self, val: T) {
        unsafe {
            let offset = self.inner.offset(self.len as isize);
            ptr::write(offset, val);
            self.len += 1;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        unsafe {
            self.len -= 1;
            let offset = self.inner.offset(self.len as isize);
            match offset.is_null() {
                true => None,
                false => Some(offset.read()),
            }
        }
    }

    pub fn into_iter(self) -> HorridIterator<T> {
        HorridIterator {
            vec: self,
            index: 0,
        }
    }

    pub unsafe fn get(&self, index: usize) -> Option<T> {
        if index > self.len {
            return None;
        }
        let offset = self.inner.offset(index as isize);
        match offset.is_null() {
            true => None,
            false => Some(offset.read()),
        }
    }
}

impl<T: fmt::Debug> Drop for HorridVec<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
        let start = self.inner.cast::<u8>();
        unsafe {
            dealloc(
                start,
                Layout::from_size_align(self.capacity * size_of::<T>(), align_of::<T>()).unwrap(),
            )
        }; // <-- can't touch this!
    }
}

impl<T: fmt::Debug> Index<usize> for HorridVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe {
            if index > self.len {
                panic!("omg what are you doing!");
            }
            let offset = self.inner.offset(index as isize);
            offset.as_ref().unwrap()
        }
    }
}

pub struct HorridIterator<T: fmt::Debug> {
    vec: HorridVec<T>,
    index: usize,
}

impl<T: fmt::Debug> HorridIterator<T> {
    unsafe fn pop_front(&mut self) -> Option<T> {
        if self.vec.len == 0 {
            return None;
        }

        let val = self.vec.inner.offset(self.vec.offset).read();
        self.vec.len -= 1;
        self.vec.offset += 1;
        Some(val)
    }
}

impl<T: fmt::Debug> Iterator for HorridIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let val = unsafe { self.pop_front() };
        self.index += 1;
        val
    }
}
