use std::fmt::Display;
use std::ptr;
use std::ops::{Index, IndexMut};
use std::ops::{Deref, DerefMut};
use crate::chunks as my;

#[derive(Debug)]
pub struct Vector<T: Display + Clone> {
    data: my::Chunks<T>,
    pub len: usize,
}


impl<T: Display + Clone + Copy> Vector<T> {
    // Constructor
    pub fn new_copy(value: T, len: usize) -> Self {
        // Allocate at least something
        let capacity = if len > 0 { len } else { 1 };

        let chunks = my::Chunks::filled_copy(value, capacity);
        Vector {
            data: chunks,
            len: len
        }
    }

    // Constructor
    pub fn from_slice_copy(from: &[T]) -> Self {
        Self {
            data: my::Chunks::from_slice_copy(from),
            len: from.len()
        }
    }

}

impl<T: Display + Clone> Vector<T> {
    pub fn new() -> Self {
        Self {
            data: my::Chunks::alloc(1),
            len: 0
        }
    }

    // Constructor
    pub fn new_clone(value: T, len: usize) -> Self {
        // Allocate at least something
        let capacity = if len > 0 { len } else { 1 };

        let chunks: my::Chunks<T> = my::Chunks::filled_clone(value, capacity);
        Self {
            data: chunks,
            len: len
        }
    }

    // Constructor
    pub fn from_slice_clone(from: &[T]) -> Self {
        Self {
            data: my::Chunks::from_slice_clone(from),
            len: from.len()
        }
    }

    // Constructor
    pub fn from_raw_parts(ptr: *mut T, len: usize, capacity: usize) -> Self {
        Self {
            data: my::Chunks {
                ptr: ptr,
                count: capacity,
            },
            len: len
        }
    }

    pub fn len_bytes(&self) -> usize {
        self.len * size_of::<T>()
    }

    pub fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    pub fn as_mut_ptr(&self) -> *mut T {
        self.data.as_mut_ptr()
    }

    pub fn as_slice(&self) -> &[T] {
        // Does it have overhead?
        // &self.data.as_slice()[0..self.len]

        // Works also. Is it better?
        // self.as_mut_slice() as &[T]

        unsafe {
            std::slice::from_raw_parts(
                self.as_ptr(),
                // Bound by len, not by data.count
                self.len
            )
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.as_mut_ptr(),
                // Bound by len, not by data.count
                self.len
            )
        }
    }

    pub fn push(&mut self, elem: T) -> bool {
        if self.len == self.data.count {
            self.data.grow(1);
        }

        self.data.write_index(self.len, elem);
        self.len += 1;
        true
    }

    pub fn insert(&mut self, index: usize, elem: T) -> bool {
        if self.len <= index {
            return false;
        }

        if self.len == self.data.count {
            self.data.grow(self.grow_strategy());
        }

        // TODO Safety: ?
        // TODO Why it introduces overhead if used in append()?
        unsafe {
            ptr::copy(
                &mut self.data[index],
                &mut self.data[index + 1],
                self.len - index,
            );
        }

        // self.data[index] = elem;
        self.data.write_index(index, elem);
        self.len += 1;
        true
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len > 0 {
            self.len -= 1;
            // Interesting: we can copy by reference, this is done automatically by compiler
            // (bad design!)
            Some(self.data[self.len].clone())
        } else {
            None
        }
    }

    pub fn capacity(&self) -> usize {
        self.data.count
    }

    // ===== Private =====

    fn bounds(&self, index: usize) -> bool {
        // TODO Turn BOUNDS_CHECK off for self.data
        0 <= index && index < self.len
    }

    /// Given the current capacity & size of an elem, returns new capacity to grow by
    fn grow_strategy(&self) -> usize {
        match self.capacity() {
            0..3 => 4,
            _ => self.capacity() * 2
        }
    }
}

// ======== CONSEC ========

pub trait ConsecConstrucor {
    fn consec(len: usize) -> Self;
}

impl<T: Display + From<usize> + Clone> ConsecConstrucor for Vector<T> {
    // Constructor
    fn consec(len: usize) -> Self {
        let mut obj = Self {
            data: my::Chunks::alloc(len),
            len: len
        };

        for i in 0..len {
            // non-primitive cast: `usize` as `T`
            // Why does it occur?
            // obj[i] = (i + 1) as T;
            obj[i] = T::from(i + 1);
        }
        obj
    }
}

// ======== INDEX ========

impl<
    T: Display + Clone,
> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if !self.bounds(index) {
            panic!("Index out of bounds");
        }

        self.data.index(index)
    }
}

impl<
    T: Display + Clone,
> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if !self.bounds(index) {
            panic!("Index out of bounds");
        }

        self.data.index_mut(index)
    }
}

// ======== DEREF ========
// Automatically implements iter(). How it works?

impl<T: Display + Clone> Deref for Vector<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T: Display + Clone> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

// ======== ITERATOR ========

impl<T: Display + Clone> FromIterator<T> for Vector<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut c = Self::new();

        for i in iter {
            c.push(i);
        }

        c
    }
}
