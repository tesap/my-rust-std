use std::fmt;
use std::ptr;
//use core::slice;
use std::ops::{Index, IndexMut};
use std::fmt::Display;
use std::slice;

// mod mem_utils;
use crate::mem_utils::{array_alloc, array_realloc, array_dealloc, array_init, array_deinit};

pub struct Chunks<T: Clone>
{
    // TODO Option<> + ? (In Safety section)
    pub ptr: *mut T,
    pub cap: usize,
    pub len: usize,
}

impl<T: Clone + Display + Default> Chunks<T> {
    pub fn new(len: usize) -> Self {
        Self::new_init(len, &T::default())
    }

}

impl<T: Clone + Display> Chunks<T> {
    pub fn new_init(len: usize, init_value: &T) -> Self {
        if (len == 0) {
            return Self {
                ptr: array_alloc::<T>(1),
                cap: 1,
                len: 0,
            };
        }

        let ptr = array_alloc::<T>(len);
        unsafe {
            array_init(ptr, len, init_value);
        }

        Self {
            ptr,
            cap: len,
            len,
        }
    }
}

impl<T: Copy> Chunks<T> {
    pub fn from_slice_copy(from: &[T]) -> Self {
        let size = from.len();
        let mut c = Self::new_empty(size);

        if size > 0 {
            unsafe {
                ptr::copy(from.as_ptr(), c.ptr, size)
            }
        }
        c
    }
}

impl<T: Clone> Chunks<T> {
    // === as_* ===
    pub fn as_slice(&self) -> &[T] {
        unsafe {
            slice::from_raw_parts(self.ptr, self.len)
        }
    }

    // === from_* ===
    // For from_slice only
    fn new_empty(len: usize) -> Self {
        let ptr = array_alloc::<T>(len);

        Self {
            ptr,
            cap: len,
            len,
        }
    }

    pub fn from_slice_clone(from: &[T]) -> Self {
        // That does a move
        let mut c = Self::new_empty(from.len());
        for i in 0..from.len() {
            unsafe {
                (&mut c[i] as *mut T).write(from[i].clone());
            }
        }
        c
    }

    // === Private ===
    fn realloc(&mut self, new_cap: usize) {
        unsafe {
            self.ptr = array_realloc(self.ptr, self.cap, new_cap);
        }
        self.cap = new_cap;
    }

    fn bounds_initialized(&self, index: usize) -> bool {
        /*0 <= index &&*/ index < self.len
    }

    fn get_index_ref(&self, index: usize) -> &T {
        if self.bounds_initialized(index) {
            unsafe {
                &*self.ptr.add(index) as &T
            }
        } else {
            panic!("Index out of bounds");
        }
    }

    fn get_index_mut_ref(&self, index: usize) -> &mut T {
        if self.bounds_initialized(index) {
            unsafe {
                &mut *self.ptr.add(index) as &mut T
            }
        } else {
            panic!("Index out of bounds");
        }
    }
}

// ================== INDEX & INDEX_MUT ==================

impl<T: Clone> Index<usize> for Chunks<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe {
            // &*self.ptr.add(index)
            self.get_index_ref(index)
        }
    }
}

impl<T: Clone> IndexMut<usize> for Chunks<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe {
            //&mut *self.ptr.add(index)
            self.get_index_mut_ref(index)
        }
    }
}

// ================== FMT ==================

impl<T: Display + Clone> fmt::Debug for Chunks<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Chunks: [").unwrap();
        for i in 0..self.cap {
            if i > 0 {
                write!(f, ", ").unwrap();
            }

            // Reference are non-destructive,
            // so we do not need to additionally worry about that object
            let val: &T = unsafe {
                &*self.ptr.add(i)
            };
        }
        write!(f, "]")
    }
}

// ================== DROP ==================

impl<T: Clone> Drop for Chunks<T> {
    fn drop(&mut self) {
        unsafe {
            // Deinit only initialized values, which are within length
            array_deinit(self.ptr, self.len);
            array_dealloc(self.ptr, self.cap);
        }
        self.cap = 0;
    }
}


// Vector API
// Constructor(empty)
// Constructor()
// get([i])
// set([i])
//
