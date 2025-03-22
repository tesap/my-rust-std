
macro_rules! test_parametrized {
    ($func_name:ident, $type_ident:ident, $type:ty) => {
        #[test]
        fn $type_ident() {
            return $func_name::<$type>();
        }
    }
}

// Limitation of Rust
//struct Cloneable{
//    pub cloned_cnt: i32 = 0
//};
//
//impl Cloneable {
//    fn new() -> Self {
//        Self {
//            cloned_cnt: 0
//        }
//    }
//}
//
//impl Clone for Cloneable {
//    fn clone(&self) -> Self {
//        self.cloned_cnt += 1;
//        Self::new()
//    }
//}

#[cfg(test)]
mod tests {
    use tesap_std::Chunks;
    use assert_panic::assert_panic;
    // use std::mem;

    #[test]
    fn test_new_init_trivial_type() {
        let mut chunks = Chunks::<u32>::new_init(3, &10);
        assert_eq!(chunks.as_slice(), &[10, 10, 10]);
    }

    #[test]
    fn test_new_init_clone_type() {
        let mut s = String::from("asdf");
        let mut chunks = Chunks::<String>::new_init(3, &s);
    }

    #[test]
    fn test_as_slice_str() {
        let mut s = String::from("asdf");
        let mut c = Chunks::<String>::new_init(3, &s);
        assert_eq!(c.as_slice(), &["asdf", "asdf", "asdf"]);
    }

    #[test]
    fn test_index_get() {
        let mut s = String::from("asdf");
        let mut c = Chunks::<String>::new_init(3, &s);

        // leads to duplication of value and double-drop
        // let s2 = c.get_index(0).read();
        // let s2 = (*c.get_index(0)).clone();
        // TODO: Add to Ref section: limitation of Rust's &
        // let s2: &String = &c[0];

        assert_eq!(c[0], "asdf");
        assert_eq!(c[1], "asdf");
        assert_eq!(c[2], "asdf");
    }

    #[test]
    fn test_index_get_mut_ref_set() {
        let mut s = String::from("asdf");
        let mut c = Chunks::<String>::new_init(3, &s);

        let s2: &mut String = &mut c[0];
        assert_eq!(s2, "asdf");

        *s2 = String::from("123");
        assert_eq!(c.as_slice(), &["123", "asdf", "asdf"]);
    }

    #[test]
    fn test_index_set() {
        // For String
        let mut s = String::from("asdf");
        let mut c = Chunks::<String>::new_init(3, &s);

        let s2 = String::from("new");
        c[0] = s2;

        assert_eq!(c.as_slice(), &["new", "asdf", "asdf"]);

        // For u32
        let mut chunks = Chunks::<u32>::new_init(3, &5);
        chunks[1] = 99;
        assert_eq!(chunks[1], 99);
    }

    #[test]
    fn test_index_out_of_bounds() {
        let mut chunks = Chunks::<u32>::new_init(3, &10);
        assert_eq!(chunks[0], 10);
        assert_eq!(chunks[1], 10);
        assert_eq!(chunks[2], 10);
        assert_panic!({ chunks[3]; });
    }

    //
    //fn test_reinterpret<T>()
    //where
    //    T: Copy + std::fmt::Debug + std::cmp::PartialEq + From<u8>
    //{
    //    let SIZE = 20;
    //    let VALUE: T = 100.into();
    //    let size_factor: usize = mem::size_of::<T>() / mem::size_of::<u8>();
    //    let mut chunks = Chunks::<T>::alloc(SIZE);
    //    chunks.memset_copy(VALUE.into());
    //    assert_eq!(chunks[0], VALUE);
    //
    //    let chunks_view: Chunks::<T, false> = Chunks {
    //        ptr: chunks.ptr,
    //        count: chunks.count
    //    };
    //    /*
    //     * Check that no further allocation happenned out of bounds
    //     */
    //    assert_ne!(chunks_view[chunks.count], VALUE);
    //    mem::forget(chunks_view);
    //
    //    let ptr = chunks.ptr as *mut u8;
    //    // BOUNDS_CHECK = false : Turn off as needed to exceed bounds intentionally further
    //    let chunks2: Chunks<u8, false> = Chunks {
    //        ptr: ptr,
    //        count: SIZE * size_factor,
    //    };
    //
    //    /*
    //     * Checking that VALUE is present in first byte of each chunk
    //     */
    //    chunks2.indices().for_each(|i| {
    //        if i % size_factor == 0 {
    //            // TODO What?!
    //            assert_eq!(<u8 as Into<T>>::into(chunks2[i]), VALUE, "(i: {i}) First byte in chunk is to be {VALUE:?}");
    //        } else {
    //            assert_eq!(<u8 as Into<T>>::into(chunks2[i]), 0.into(), "(i: {i}) Rest part of chunk is to be 0");
    //        }
    //    });
    //    // DROP = false : Double-free is possible, so do not treat it as allocated
    //    mem::forget(chunks2);
    //}
    //
    //test_parametrized!(test_reinterpret, test_reinterpret_u8, u8);
    //test_parametrized!(test_reinterpret, test_reinterpret_u16, u16);
    //test_parametrized!(test_reinterpret, test_reinterpret_u32, u32);
    //test_parametrized!(test_reinterpret, test_reinterpret_i16, i16);
    //test_parametrized!(test_reinterpret, test_reinterpret_i32, i32);
    //test_parametrized!(test_reinterpret, test_reinterpret_i64, i64);


    #[test]
    fn test_chunks_to_std_vec_from_raw_parts() {
        // For u8
        let mut v: Vec<u8> = Vec::new();

        v.push(20);
        v.push(20);
        v.push(23);
        v.push(23);
        v.push(28);
        v.push(20);
        v[0] = 10;
        v.push(22);
        v.shrink_to_fit();
        v.push(21);

        let mut c: Chunks<u8> = Chunks {
            ptr: v.as_mut_ptr(),
            len: v.len(),
            cap: v.len(),
        };

        assert_eq!(v.as_slice(), c.as_slice());
        std::mem::forget(c);

        // For String
        let mut v: Vec<String> = Vec::new();
        v.push(String::from("abc"));
        v.push(String::from("def"));
        v.push(String::from("ghi"));

        let mut v_view: Chunks<String> = Chunks {
            ptr: v.as_mut_ptr(),
            len: v.len(),
            cap: v.len(),
        };

        // Drop by the means of Chunks, not Vec
        std::mem::forget(v);
    }

    #[test]
    fn test_as_slice() {
        let mut chunks = Chunks::<u8>::new_init(3, &1);
        // What is &[...] notation? Does it create object on memory?
        assert_eq!(chunks.as_slice(), &[1, 1, 1]);
        // assert_eq!(chunks.as_mut_slice(), &[1, 1, 1]);

        chunks[1] = 10;
        assert_eq!(chunks.as_slice(), &[1, 10, 1]);
    }

    #[test]
    fn test_debug() {
        let c = Chunks::<u8>::new_init(3, &1);
        println!("Debug: {:?}", c);

        assert_eq!(c.as_slice(), &[1, 1, 1]);

        // Tests a Debug trait with a T being 'Clone'.
        // This is important because trying to format a T object located in memory
        // without disabling auto-drop, will lead to objects destructors being called
        // in fmt() function.
        let c: Chunks<String> = Chunks::new_init(5, &"123".to_string());
        assert_eq!(c.as_slice(), &["123", "123", "123", "123", "123"]);
        println!("Debug: {:?}", c);

        assert_eq!(c.as_slice(), &["123", "123", "123", "123", "123"]);
    }

    // TEST &str
    #[test]
    fn test_str() {
        // FROM_SLICE (COPY)
        let c: Chunks<&str> = Chunks::from_slice_clone(&["x", "y", "z"]);
        assert_eq!(c.as_slice(), &["x", "y", "z"]);
        std::mem::forget(c);

        let c: Chunks<&str> = Chunks::from_slice_copy(&["x", "y", "z"]);
        assert_eq!(c.as_slice(), &["x", "y", "z"]);
        std::mem::forget(c);
    }

    // TEST String
    #[test]
    fn test_std_string() {
        // FROM_SLICE (CLONE)
        let mut c: Chunks<String> = Chunks::from_slice_clone(&["x".to_string(), "y".to_string(), "z".to_string()]);
        //assert_eq!(c.as_slice(), &["x", "y", "z"]);
        //std::mem::forget(c);
    }

}

