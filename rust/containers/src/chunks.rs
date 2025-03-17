use std::alloc;
use std::ptr;
use std::mem;
use std::fmt;
use core::slice;
use std::ops::{Index, IndexMut};
use std::fmt::Display;

type Layout = alloc::Layout;

fn array_layout<T>(count: usize) -> Layout {
    let layout = alloc::Layout::array::<T>(count).unwrap();
    assert_ne!(layout.size(), 0);
    assert_eq!(layout.size(), count * mem::size_of::<T>());
    layout
}

// TODO Add Option for error case
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


pub struct Chunks<T: Clone, const BOUNDS_CHECK: bool = true>
{
    pub ptr: *mut T,
    pub count: usize,
}

impl<
    T: Copy + Clone,
    const BC: bool,
> Chunks<T, BC> {

    pub fn memset_copy(&mut self, value: T) {
        for i in 0..self.count {
            // ptr::write(self.ptr.add(i), value);
            self[i] = value;
        }
    }

    // Constructor
    pub fn filled_copy(value: T, count: usize) -> Self {
        let mut c: Self = Self::alloc(count);
        c.memset_copy(value);
        c
    }
}

impl<
    T: Clone + Display,
    const BC: bool,
> Chunks<T, BC> {
    pub fn memset_clone(&mut self, value: T) {
        for i in 0..self.count {
            // Err: invalid memory reference
            // self[i] = value.clone();

            self.write_ptr(i, value.clone());
        }
    }

    // Constructor
    // TODO Can we reuse one by another
    pub fn filled_clone(value: T, count: usize) -> Self {
        let mut c: Self = Self::alloc(count);
        c.memset_clone(value);
        c
    }

}


impl<
    T: Clone,
    const BC: bool,
> Chunks<T, BC> {
    // Constructor
    pub fn alloc(count: usize) -> Self {
        Self {
            ptr: array_alloc::<T>(count),
            count,
        }
    }

    pub fn from_slice(from: &[T]) -> Self {
        let count: usize = from.len();
        let _self = Self::alloc(count);

        if count > 0 {
            unsafe {
                ptr::copy(from.as_ptr(), _self.ptr, count)
            }
        }
        _self
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


    /// Replaces mutable acces via index like `self.data[self.len] = elem`
    /// by providing an access to an element via pointer `*mut T`
    /// rather than reference `&mut T`
    ///
    /// This is made for objects with custom Drop trait (such as String):
    /// Trying to overwrite memory location by `&T` reference (returned by index_mut)
    /// causes invokation of drop().
    /// But this is unwanted behaviour for memory without an actual object
    /// and will lead to "invalid memory reference" error in this case.
    pub fn write_ptr(&mut self, index: usize, value: T) {
        // TODO match
        let p: *mut T = self.get_mut_ptr(index).unwrap();
        unsafe {
            p.write(value);
        }
    }

    // === PRIVATE ===
    fn get_ptr(&self, index: usize) -> Result<*const T, &'static str> {
        // Safety: Out-of-bounds is checked
        if self.bounds(index) {
            unsafe {
                Ok(self.ptr.add(index) as *const T)
            }
        } else {
            Err("Index out of bounds")
        }
    }

    fn get_mut_ptr(&mut self, index: usize) -> Result<*mut T, &'static str> {
        // Safety: Out-of-bounds is checked
        if self.bounds(index) {
            unsafe {
                Ok(self.ptr.add(index))
            }
        } else {
            Err("Index out of bounds")
        }
    }

    //
    //fn get(&self, index: usize) -> Result<&T, &'static str> {
    //    // Safety: Out-of-bounds is checked
    //    if self.bounds(index) {
    //        unsafe {
    //            // TODO Maybe return by reference?
    //            // Does &mut *... is a borrowing, i.e. moving?
    //            // Looks like overhead of moving twice
    //            Ok(&*self.ptr.add(index))
    //        }
    //    } else {
    //        Err("Index out of bounds")
    //    }
    //}

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
// TODO Seems that cannot use self[i] = string1; // (String)
// Because returning a reference to a non-existing type value

impl<
    T: Clone,
    const BC: bool,
> Index<usize> for Chunks<T, BC> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe {
            &*self.get_ptr(index).unwrap()
        }
    }
}

impl<
    T: Clone,
    const BC: bool,
> IndexMut<usize> for Chunks<T, BC> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe {
            &mut *self.get_mut_ptr(index).unwrap()
        }
    }
}

// ================== FMT ==================

impl<
    T: Display + Clone,
    const BC: bool,
> fmt::Debug for Chunks<T, BC> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Chunks: [").unwrap();
        for i in 0..self.count {

            if i > 0 {
                write!(f, ", ").unwrap();
            }
            let val: T = unsafe {
                self.ptr.add(i).read()
            };
            write!(f, "{}", val).unwrap();
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
        if self.allocated() {
            self.dealloc();
        }
    }
}
