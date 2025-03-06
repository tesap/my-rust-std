use std::alloc;
use std::ptr;
use std::mem;
use std::fmt;
use core::slice;
use std::ops::{Index, IndexMut};

type Layout = alloc::Layout;

fn array_layout<T>(count: usize) -> Layout {
    let layout = alloc::Layout::array::<T>(count).unwrap();
    assert_ne!(layout.size(), 0);
    assert_eq!(layout.size(), count * mem::size_of::<T>());
    layout
}

fn array_alloc<T>(count: usize) -> *mut T {
    let layout = array_layout::<T>(count);

    unsafe {
        alloc::alloc(layout) as *mut T
    }
}

fn array_realloc<T>(ptr: *mut T, count: usize, new_count: usize) -> *mut T {
    if new_count == count {
        return ptr;
    }

    let layout = array_layout::<T>(count);

    unsafe {
        alloc::realloc(
            ptr as *mut u8,
            layout,
            array_layout::<T>(new_count).size()
        ) as *mut T
    }
}

fn array_dealloc<T>(ptr: *mut T, count: usize) {
    // Safety: memory was allocated with same pointer and layout alignment
    unsafe {
        alloc::dealloc(
            ptr as *mut u8,
            array_layout::<T>(count)
        )
    }
}


pub struct Chunks<T, const BOUNDS_CHECK: bool = true, const AUTO_DROP: bool = true>
where
    T: Copy
{
    pub ptr: *mut T,
    pub count: usize,
}

impl<
    T: Copy,
    const BC: bool,
    const AD: bool,
> Chunks<T, BC, AD> {
    // Constructor
    pub fn alloc(count: usize) -> Self {
        Self {
            ptr: array_alloc::<T>(count),
            count,
        }
    }

    // Constructor
    pub fn from_slice(from: &[T]) -> Self {
        let _self: Self = Self::alloc(from.len());
        for i in 0..from.len() {
            unsafe {
                _self.ptr.add(i).write(from[i]);
            }
        }
        _self
    }

    // Constructor
    pub fn filled(value: T, count: usize) -> Self {
        let mut c: Self = Self::alloc(count);
        c.memset(value);
        c
    }

    pub fn dealloc(&mut self) {
        if self.allocated() {
            array_dealloc(self.ptr, self.count);
        }

        self.ptr = ptr::null::<T>() as *mut T;
        self.count = 0;
    }

    pub fn realloc(&mut self, new_count: usize) {
        if self.allocated() {
            self.ptr = array_realloc(self.ptr, self.count, new_count);
        } else {
            self.ptr = array_alloc(new_count);
        }
        self.count = new_count;
    }

    pub fn grow(&mut self, delta: usize) {
        if !self.allocated() {
            // Copy is in action? How efficiently?
            // self = Self::alloc(delta);
            return;
        }
        self.realloc(self.count + delta);
    }

    pub fn allocated(&self) -> bool {
        !self.ptr.is_null() && self.count > 0
    }

    pub fn memset(&mut self, value: T) {
        for i in 0..self.count {
            // ptr::write(self.ptr.add(i), value);
            self[i] = value;
        }
    }

    pub fn as_ptr(&self) -> *const T {
        self.ptr as *const T
    }

    pub fn as_mut_ptr(&self) -> *mut T {
        self.ptr
    }

    pub fn as_array<const N: usize>(&self) -> Option<&[T; N]> {
        if N != self.count {
            None
        } else {
            // Doesnt work
            // &self.ptr as &[T; N]
            let ptr = self.as_ptr() as *const [T; N];
            unsafe {
                Some(&*ptr)
            }
        }
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.ptr, self.count) }
    }

    pub fn indices(&self) -> std::ops::Range<usize> {
        0..self.count
    }

    // Private
    fn bounds(&self, index: usize) -> bool {
        match BC {
            false => true,
            true => 0 <= index && index < self.count,
        }
    }

    fn get(&self, index: usize) -> Result<&T, &'static str> {
        // Safety: Out-of-bounds is checked
        if self.bounds(index) {
            unsafe {
                // TODO Maybe return by reference?
                // Does &mut *... is a borrowing, i.e. moving?
                // Looks like overhead of moving twice
                Ok(&*self.ptr.add(index))
            }
        } else {
            Err("Index out of bounds")
        }
    }

    fn get_mut(&mut self, index: usize) -> Result<&mut T, &'static str> {
        // TODO Check overheads of such solution
        // May it be done better?
        //match self.get(index) {
        //    // === Error: [E0605]: non-primitive cast: `&T` as `&mut T`
        //    Ok(value) => Ok(value as &mut T),
        //    Err(err) => panic!("{}", err),
        //}

        if self.bounds(index) {
            // Safety: Out-of-bounds is checked
            unsafe {
                Ok(&mut *self.ptr.add(index))
            }
        } else {
            Err("Index out of bounds")
        }
    }
    // TODO is it right approach in Rust to have mut & const function's duplicates?
}

// ================== INDEX & INDEX_MUT ==================

impl<
    T: Copy,
    const BC: bool,
    const AD: bool,
> Index<usize> for Chunks<T, BC, AD> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<
    T: Copy,
    const BC: bool,
    const AD: bool,
> IndexMut<usize> for Chunks<T, BC, AD> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

// ================== FMT ==================

impl<
    T: fmt::Display + Copy,
    const BC: bool,
    const AD: bool,
> fmt::Debug for Chunks<T, BC, AD> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Chunks: [");
        for i in 0..self.count + 20 {
            if i > 0 { write!(f, ", "); }
            let val: T = unsafe { self.ptr.add(i).read() };
            write!(f, "{}", val);
        }
        write!(f, "]")
    }
}

// ================== DROP ==================

impl<
    T: Copy,
    const BC: bool,
    const AD: bool,
> Drop for Chunks<T, BC, AD> {
    fn drop(&mut self) {
        if self.allocated() && AD {
            self.dealloc();
        }
    }
}
