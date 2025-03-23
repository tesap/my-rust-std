use std::fmt;
use std::ptr;
use std::ops::{Index, IndexMut};
use std::ops::{Deref, DerefMut};
use std::fmt::Display;
use std::mem;
use std::slice;

use crate::mem_utils::{array_alloc, array_realloc, array_dealloc, array_init, array_deinit, array_write_drop, array_write_no_drop};

pub struct Vector<T: Clone>
{
    // TODO Option<> + ? (In Safety section)
    pub ptr: *mut T,
    pub cap: usize,
    pub len: usize,
}

impl<T: Clone + Display + Default> Vector<T> {
    pub fn new(len: usize) -> Self {
        Self::new_init(len, &T::default())
    }

}

impl<T: Clone + Display> Vector<T> {
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

impl<T: Copy> Vector<T> {
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

impl<T: Clone> Vector<T> {
    // === as_* ===
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            slice::from_raw_parts_mut(self.ptr, self.len)
        }
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe {
            slice::from_raw_parts(self.ptr, self.len)
        }
    }

    pub unsafe fn as_ptr(&self) -> *const T {
        self.ptr as *const T
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr
    }
    // === ===

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

    // === Public ===
    pub fn push(&mut self, elem: T) -> bool {
        if self.len == self.cap {
            self.realloc(self.cap * 3);
        }

        // The self.ptr[self.len] is NOT initialized,
        // so we should not drop the old value
        unsafe {
            array_write_no_drop(self.ptr, self.len, elem);
        }
        self.len += 1;
        true
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        // Interesting: we can copy by reference, this is done automatically by compiler
        // (bad design!)

        let index = self.len - 1;
        let val: T = self.get_index_ref(index)?.clone();
        self.len -= 1;

        Some(val)
    }

    pub fn capacity(&self) -> usize {
        self.cap
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn len_bytes(&self) -> usize {
        self.len * mem::size_of::<T>()
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

    fn get_index_ref(&self, index: usize) -> Option<&T> {
        if self.bounds_initialized(index) {
            // SAFETY: We're sure that the value at self.ptr[index] is initialized
            // because it's withing self.length
            unsafe {
                Some(&*self.ptr.add(index) as &T)
            }
        } else {
            None
        }
    }

    fn get_index_mut_ref(&self, index: usize) -> Option<&mut T> {
        if self.bounds_initialized(index) {
            unsafe {
                Some(&mut *self.ptr.add(index) as &mut T)
            }
        } else {
            None
        }
    }
}

// ================== INDEX & INDEX_MUT ==================

impl<T: Clone> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        // --> Also possible
        // &*self.ptr.add(index)

        match self.get_index_ref(index) {
            Some(v) => v,
            None => panic!("Index out of bounds"),
        }
    }
}

impl<T: Clone> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        // --> Also possible
        //&mut *self.ptr.add(index)

        match self.get_index_mut_ref(index) {
            Some(v) => v,
            None => panic!("Index out of bounds"),
        }
    }
}

// ================== FMT ==================

impl<T: Display + Clone> fmt::Debug for Vector<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vector: [").unwrap();
        for i in 0..self.cap {
            if i > 0 {
                write!(f, ", ").unwrap();
            }

            // Reference are non-destructive,
            // so we do not need to additionally worry about that object
            let val: &T = unsafe {
                &*self.ptr.add(i)
            };
            write!(f, "{:}", val);
        }
        write!(f, "]")
    }
}

// ================== DROP ==================

impl<T: Clone> Drop for Vector<T> {
    fn drop(&mut self) {
        unsafe {
            // Deinit only initialized values, which are within length
            array_deinit(self.ptr, self.len);
            array_dealloc(self.ptr, self.cap);
        }
        self.cap = 0;
    }
}

// ======== DEREF ========
// Automatically implements iter(). 
// TODO How it works?

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

impl<T: Display + Clone + Default> FromIterator<T> for Vector<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut c = Self::new(0);

        for i in iter {
            c.push(i);
        }

        c
    }
}

// ======== FROM & INTO ========

impl<T: Display + Clone> From<Vec<T>> for Vector<T> {
    fn from(mut from: Vec<T>) -> Self {
        // Disable drop of Vec
        // Leads to double-free without this
        let mut from = mem::ManuallyDrop::new(from);

        // SAFETY: We disable drop of the `from` object,
        // so that ptr can be passed safely
        unsafe {
            Vector {
                ptr: from.as_mut_ptr(),
                cap: from.capacity(),
                len: from.len(),
            }
        }
        // Or without this
        // std::mem::forget(from);
    }
}

impl<T: Display + Clone> Into<Vec<T>> for Vector<T> {
    fn into(mut self) -> Vec<T> {
        // Overhead because of `memcpy`
        //let mut _self = mem::ManuallyDrop::new(self);

        let c = unsafe {
            Vec::from_raw_parts(
                self.as_mut_ptr(),
                self.len(),
                self.capacity()
            )
        };
        std::mem::forget(self);
        c
    }
}

// Vector API
// Constructor(empty)
// Constructor()
// get([i])
// set([i])
//
