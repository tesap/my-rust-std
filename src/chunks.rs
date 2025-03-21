use std::ptr;
//use std::mem;
use std::fmt;
//use core::slice;
use std::ops::{Index, IndexMut};
use std::fmt::Display;

// mod mem_utils;
use crate::mem_utils::{array_alloc, array_realloc, array_dealloc};

pub struct Chunks<T: Clone, const BOUNDS_CHECK: bool = true>
{
    // TODO Option<> + ? (In Safety section)
    pub ptr: *mut T,
    pub cap: usize,
    pub len: usize,
}

impl<
    T: Clone + Display,
    const BC: bool,
> Chunks<T, BC> {
    pub fn new() -> Self {
        Self {
            ptr: array_alloc::<T>(1),
            cap: 1,
            len: 0,
        }
    }
}

impl<
    T: Clone,
    const BC: bool,
> Chunks<T, BC> {
    pub fn dealloc(&mut self) {
        if !self.allocated() {
            panic!("Could not dealloc not allocated");
            return;
        }

        // Dropping initialized values
        for i in 0..self.len {
            unsafe {
                ptr::drop_in_place(self.ptr.add(i) as *mut T);
            }
        }

        array_dealloc(self.ptr, self.cap);
        self.cap = 0;
    }

    pub fn realloc(&mut self, new_cap: usize) {
        if !self.allocated() {
            panic!("Not allocated for realloc");
        }

        self.ptr = array_realloc(self.ptr, self.cap, new_cap);
        self.cap = new_cap;
    }

    pub fn grow(&mut self, delta: usize) {
        if !self.allocated() {
            return;
        }
        self.realloc(self.cap + delta);
    }

    // === Private ===
    fn bounds(&self, index: usize) -> bool {
        match BC {
            false => true,
            true => /*0 <= index &&*/ index < self.len,
        }
    }

    pub fn allocated(&self) -> bool {
        self.cap > 0
    }
}

// ================== FMT ==================

impl<
    T: Display + Clone,
    const BC: bool,
> fmt::Debug for Chunks<T, BC> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Chunks: [").unwrap();
        for i in 0..self.cap {
            if i > 0 {
                write!(f, ", ").unwrap();
            }

            // Todo replace with val = *ptr
            let val: T = unsafe {
                // Move
                self.ptr.add(i).read()
            };
            write!(f, "{}", val).unwrap();

            // --> We don't wont to drop a value
            std::mem::forget(val);
        }
        write!(f, "]")
    }
}

// ================== DROP ==================

impl<
    T: Clone,
    const BC: bool,
> Drop for Chunks<T, BC> {
    fn drop(&mut self) {
        println!("CHUNKS DROP: {:}, {:}", self.len, self.cap);
        if self.allocated() {
            self.dealloc();
        }
    }
}
