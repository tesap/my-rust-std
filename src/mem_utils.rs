use std::alloc;
use std::mem;
use std::ptr;

type Layout = alloc::Layout;


pub fn array_layout<T>(size: usize) -> Layout {
    let layout = alloc::Layout::array::<T>(size).unwrap();
    assert_ne!(layout.size(), 0);
    assert_eq!(layout.size(), size * mem::size_of::<T>());
    layout
}

pub fn array_alloc<T>(size: usize) -> *mut T {
    let layout = array_layout::<T>(size);
    println!("ARRAY_ALLOC: {:}", size);
    unsafe {
        alloc::alloc(layout) as *mut T
    }
}


// Safety: memory was allocated with same layout
pub unsafe fn array_dealloc<T>(ptr: *mut T, size: usize) {
    println!("ARRAY_DEALLOC: {:}", size);
    let layout = array_layout::<T>(size);
    alloc::dealloc(
        ptr as *mut u8,
        layout
    )
}

// Initialize each element in the allocated memory
pub unsafe fn array_init<T: Clone>(ptr: *mut T, size: usize, init_value: &T) {
    println!("ARRAY_INIT: {:}", size);
    for i in 0..size {
        ptr::write(ptr.add(i), init_value.clone());
    }
}

//pub fn array_alloc_default<T: Default + Clone>(size: usize) -> *mut T {
//    let ptr = array_alloc(size);
//    unsafe {
//        array_init(ptr, size, &T::default());
//    }
//    ptr
//}


// Dropping initialized values
pub unsafe fn array_deinit<T>(ptr: *mut T, size: usize) {
    println!("ARRAY_DEINIT: {:}", size);
    for i in 0..size {
        ptr::drop_in_place(ptr.add(i) as *mut T);
    }
}

// Safety: There is actual memory allocated in `ptr` with the given `layout`
pub unsafe fn array_realloc<T>(ptr: *mut T, size: usize, new_size: usize) -> *mut T {
    if size == new_size {
        return ptr;
    }
    let layout = array_layout::<T>(size);
    let new_size_bytes = mem::size_of::<T>() * new_size;

    unsafe {
        alloc::realloc(
            ptr as *mut u8,
            layout,
            new_size_bytes,
        ) as *mut T
    }
}

// Not &T, because clone is done outside if needed
pub unsafe fn array_write<T>(ptr: *mut T, index: usize, value: T) {
    // Moves `value` here
    ptr.add(index).write(value);
}

// Safety: a caller must ensure there initialized value at ptr[index]
pub unsafe fn array_write_drop<T>(ptr: *mut T, index: usize, value: T) {
    *ptr.add(index) = value
}


