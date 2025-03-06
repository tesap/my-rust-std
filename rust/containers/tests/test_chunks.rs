
macro_rules! test_parametrized {
    ($func_name:ident, $type_ident:ident, $type:ty) => {
        #[test]
        fn $type_ident() {
            return $func_name::<$type>();
        }
    }
}

#[cfg(test)]
mod tests {
    use containers::chunks::Chunks;
    use assert_panic::assert_panic;
    use std::alloc;
    use std::ptr;
    use std::mem;

    #[test]
    fn test_alloc_dealloc() {
        let mut chunks = Chunks::<u32>::alloc(10);
        assert!(!chunks.ptr.is_null());
        chunks.dealloc();
        assert!(chunks.ptr.is_null());
        assert_eq!(chunks.count, 0);
    }

    #[test]
    fn test_from_slice() {
        //                        How does it look on IR level?
        Chunks::<u8>::from_slice(&[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_memset() {
        let mut chunks = Chunks::<u32>::alloc(5);
        chunks.memset(42);
        for i in 0..5 {
            unsafe {
                assert_eq!(*chunks.ptr.add(i), 42);
            }
        }
    }

    #[test]
    fn test_index() {
        let mut chunks = Chunks::<u32>::alloc(3);
        chunks.memset(10);
        assert_eq!(chunks[0], 10);
        assert_eq!(chunks[1], 10);
        assert_eq!(chunks[2], 10);
    }

    #[test]
    fn test_index_out_of_bounds() {
        let mut chunks = Chunks::<u32>::alloc(3);
        chunks.memset(10);
        assert_eq!(chunks[0], 10);
        assert_eq!(chunks[1], 10);
        assert_eq!(chunks[2], 10);
        assert_panic!({ chunks[3]; });
    }

    #[test]
    fn test_index_mut() {
        let mut chunks = Chunks::<u32>::alloc(3);
        chunks.memset(5);
        chunks[1] = 99;
        assert_eq!(chunks[1], 99);
    }

    fn test_reinterpret<T>()
    where
        T: Copy + std::fmt::Debug + std::cmp::PartialEq + From<u8>
    {
        let SIZE = 20;
        let VALUE: T = 100.into();
        let size_factor: usize = mem::size_of::<T>() / mem::size_of::<u8>();
        let mut chunks = Chunks::<T>::alloc(SIZE);
        chunks.memset(VALUE.into());
        assert_eq!(chunks[0], VALUE);

        let chunks_view: Chunks::<T, false, false> = Chunks {
            ptr: chunks.ptr,
            count: chunks.count
        };
        /*
         * Check that no further allocation happenned out of bounds
         */
        assert_ne!(chunks_view[chunks.count], VALUE);

        let ptr = chunks.ptr as *mut u8;
        // BOUNDS_CHECK = false : Turn off as needed to exceed bounds intentionally further
        // AUTO_DROP = false : Double-free is possible, so do not treat it as allocated
        let chunks2: Chunks<u8, false, false> = Chunks {
            ptr: ptr,
            count: SIZE * size_factor,
        };

        /*
         * Checking that VALUE is present in first byte of each chunk
         */
        chunks2.indices().for_each(|i| {
            if i % size_factor == 0 {
                // TODO What?!
                assert_eq!(<u8 as Into<T>>::into(chunks2[i]), VALUE, "(i: {i}) First byte in chunk is to be {VALUE:?}");
            } else {
                assert_eq!(<u8 as Into<T>>::into(chunks2[i]), 0.into(), "(i: {i}) Rest part of chunk is to be 0");
            }
        });
    }

    test_parametrized!(test_reinterpret, test_reinterpret_u8, u8);
    test_parametrized!(test_reinterpret, test_reinterpret_u16, u16);
    test_parametrized!(test_reinterpret, test_reinterpret_u32, u32);
    test_parametrized!(test_reinterpret, test_reinterpret_i16, i16);
    test_parametrized!(test_reinterpret, test_reinterpret_i32, i32);
    test_parametrized!(test_reinterpret, test_reinterpret_i64, i64);

    #[test]
    fn test_chunks_to_vec() {
        let mut chunks;
        let mut v = unsafe {
            chunks = Chunks::<u8, false, false>::filled(1, 3);
            Vec::from_raw_parts(
                chunks.ptr,
                chunks.count,
                chunks.count,
            )
        };

        unsafe {
            assert_eq!(v.as_mut_ptr(), chunks.ptr);
        }

        v[0] = 10;
        v.push(20);
        v.push(20);
        v.push(23);
        v.push(23);
        v.push(28);
        v.push(20);
        v.push(22);
        v.shrink_to_fit();
        v.push(21);

        // Is it guaranteed?
        unsafe {
            assert_eq!(v.as_mut_ptr(), chunks.ptr);
        }

        for i in 0..v.len() {
            assert_eq!(v[i], chunks[i]);
        }
    }

    #[test]
    fn test_grow() {
        let mut chunks = Chunks::<u8>::alloc(10);
        chunks.grow(1);
        assert_eq!(chunks.count, 10 + 1);
        chunks.grow(1);
        assert_eq!(chunks.count, 10 + 2);
        chunks.grow(1);
        assert_eq!(chunks.count, 10 + 3);
    }

    #[test]
    fn test_as_slice() {
        let mut chunks = Chunks::<u8>::filled(1, 3);
        // What is &[...] notation? Does it create object on memory?
        assert_eq!(chunks.as_slice(), &[1, 1, 1]);

        chunks[1] = 10;
        assert_eq!(chunks.as_slice(), &[1, 10, 1]);
        // TODO more complicated checks
    }

    #[test]
    fn test_as_ptr() {
        let mut chunks = Chunks::<u8>::filled(1, 3);
        let ptr: *const u8 = chunks.as_ptr();

        unsafe {
            assert_eq!(*ptr, 1);
            assert_eq!(*ptr.add(1), 1);
            assert_eq!(*ptr.add(2), 1);
        }
    }

    #[test]
    fn test_as_mut_ptr() {
        let mut chunks = Chunks::<u8>::filled(1, 3);
        let ptr: *mut u8 = chunks.as_mut_ptr();

        unsafe {
            assert_eq!(*ptr, 1);
            assert_eq!(*ptr.add(1), 1);
            assert_eq!(*ptr.add(2), 1);
            ptr.write(10);
            ptr.add(1).write(20);
            ptr.add(2).write(30);
        }
        assert_eq!(chunks.as_slice(), &[10, 20, 30]);
    }

    #[test]
    fn test_as_array() {
        let mut chunks = Chunks::<u32>::alloc(3);
        chunks.memset(7);
        let array: &[u32; 3] = chunks.as_array::<3>().unwrap();
        // unsafe {
        //     // Referencing array results slice (what?)
        //     // let array_ref = &*array_ptr;
        // }
        assert_eq!(array, &[7, 7, 7]);
    }

}

