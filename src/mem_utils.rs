use std::alloc;
use std::mem;

type Layout = alloc::Layout;

fn array_layout<T>(count: usize) -> Layout {
    let layout = alloc::Layout::array::<T>(count).unwrap();
    assert_ne!(layout.size(), 0);
    assert_eq!(layout.size(), count * mem::size_of::<T>());
    layout
}

// TODO Add Option for error case
pub fn array_alloc<T>(count: usize) -> *mut T {
    let layout = array_layout::<T>(count);

    unsafe {
        alloc::alloc(layout) as *mut T
    }
}

pub fn array_realloc<T>(ptr: *mut T, count: usize, new_count: usize) -> *mut T {
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

pub fn array_dealloc<T>(ptr: *mut T, count: usize) {
    // Safety: memory was allocated with same pointer and layout alignment
    unsafe {
        alloc::dealloc(
            ptr as *mut u8,
            array_layout::<T>(count)
        )
    }
}


