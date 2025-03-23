
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
    use assert_panic::assert_panic;
    use tesap_std::Vector;

    // === Constructors
    #[test]
    fn test_new_init_trivial_type() {
        let mut v = Vector::<u32>::new_init(3, &10);
        assert_eq!(v.as_slice(), &[10, 10, 10]);
        assert_eq!(v.len(), 3);
    }

    #[test]
    fn test_new_init_clone_type() {
        let mut s = String::from("asdf");
        let mut v = Vector::<String>::new_init(3, &s);
    }

    #[test]
    fn test_new_empty() {
        let mut v: Vector<i32> = Vector::new(0);
        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), 1);
    }

    // === Operations
    #[test]
    fn test_pop_till_empty() {
        let mut v = Vector::new_init(3, &1);
        assert_eq!(v.len(), 3);
        assert_eq!(v.pop(), Some(1));
        assert_eq!(v.len(), 2);
        assert_eq!(v.pop(), Some(1));
        assert_eq!(v.len(), 1);
        assert_eq!(v.pop(), Some(1));
        assert_eq!(v.len(), 0);
        assert_eq!(v.pop(), None);
    }

    #[test]
    fn test_push_empty() {
        let mut v = Vector::new_init(0, &0);
        assert_eq!(v.push(1), true);
        assert_eq!(v.len(), 1);
        assert_eq!(v.capacity(), 1);
    }

    #[test]
    fn test_push_multiple() {
        let mut v = Vector::new_init(5, &10);

        assert_eq!(v.push(100), true);
        assert_eq!(v.len, 6);
        assert_eq!(v[v.len - 1], 100);
        assert_eq!(v.push(101), true);
        assert_eq!(v.len, 7);
        assert_eq!(v[v.len - 1], 101);
        assert_eq!(v.push(102), true);
        assert_eq!(v.len, 8);
        assert_eq!(v[v.len - 1], 102);
    }

    // === Index
    #[test]
    fn test_as_slice_str() {
        let mut s = String::from("asdf");
        let mut c = Vector::<String>::new_init(3, &s);
        assert_eq!(c.as_slice(), &["asdf", "asdf", "asdf"]);
    }

    #[test]
    fn test_index_get() {
        let mut s = String::from("asdf");
        let mut c = Vector::<String>::new_init(3, &s);

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
        let mut c = Vector::<String>::new_init(3, &s);

        let s2: &mut String = &mut c[0];
        assert_eq!(s2, "asdf");

        *s2 = String::from("123");
        assert_eq!(c.as_slice(), &["123", "asdf", "asdf"]);
    }

    #[test]
    fn test_index_set() {
        // For String
        let mut s = String::from("asdf");
        let mut c = Vector::<String>::new_init(3, &s);

        let s2 = String::from("new");
        c[0] = s2;

        assert_eq!(c.as_slice(), &["new", "asdf", "asdf"]);

        // For u32
        let mut v = Vector::<u32>::new_init(3, &5);
        v[1] = 99;
        assert_eq!(v[1], 99);
    }

    #[test]
    fn test_index_out_of_bounds() {
        let mut v = Vector::<u32>::new_init(3, &10);
        assert_eq!(v[0], 10);
        assert_eq!(v[1], 10);
        assert_eq!(v[2], 10);
        assert_panic!({ v[3]; });
    }

    // === as_slice
    
    #[test]
    fn test_as_slice() {
        let mut v = Vector::<u8>::new_init(3, &1);
        // What is &[...] notation? Does it create object on memory?
        assert_eq!(v.as_slice(), &[1, 1, 1]);
        // assert_eq!(v.as_mut_slice(), &[1, 1, 1]);

        v[1] = 10;
        assert_eq!(v.as_slice(), &[1, 10, 1]);
    }

    #[test]
    fn test_as_slice_operations() {
        let mut v = Vector::new_init(5, &1);
        assert_eq!(v.as_slice(), &[1, 1, 1, 1, 1]);

        v.push(10);
        assert_eq!(v.as_slice(), &[1, 1, 1, 1, 1, 10]);

        v.push(10);
        assert_eq!(v.as_slice(), &[1, 1, 1, 1, 1, 10, 10]);

        assert_eq!(v.pop(), Some(10));
        assert_eq!(v.as_slice(), &[1, 1, 1, 1, 1, 10]);

        v[1] = 100;
        assert_eq!(v.as_slice(), &[1, 100, 1, 1, 1, 10]);
    }

    // === Interop with std containers
    #[test]
    fn test_vector_to_std_vec_from_raw_parts() {
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

        let mut c: Vector<u8> = Vector {
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

        let mut v_view: Vector<String> = Vector {
            ptr: v.as_mut_ptr(),
            len: v.len(),
            cap: v.len(),
        };

        // Drop by the means of Vector, not Vec
        std::mem::forget(v);
    }

    #[test]
    fn test_debug() {
        let c = Vector::<u8>::new_init(3, &1);
        println!("Debug: {:?}", c);

        assert_eq!(c.as_slice(), &[1, 1, 1]);

        // Tests a Debug trait with a T being 'Clone'.
        // This is important because trying to format a T object located in memory
        // without disabling auto-drop, will lead to objects destructors being called
        // in fmt() function.
        let c: Vector<String> = Vector::new_init(5, &"123".to_string());
        assert_eq!(c.as_slice(), &["123", "123", "123", "123", "123"]);
        println!("Debug: {:?}", c);

        assert_eq!(c.as_slice(), &["123", "123", "123", "123", "123"]);
    }

    // TEST &str
    #[test]
    fn test_str() {
        // FROM_SLICE (COPY)
        let c: Vector<&str> = Vector::from_slice_clone(&["x", "y", "z"]);
        assert_eq!(c.as_slice(), &["x", "y", "z"]);
        std::mem::forget(c);

        let c: Vector<&str> = Vector::from_slice_copy(&["x", "y", "z"]);
        assert_eq!(c.as_slice(), &["x", "y", "z"]);
        std::mem::forget(c);
    }

    // TEST String
    #[test]
    fn test_std_string() {
        // FROM_SLICE (CLONE)
        let mut c: Vector<String> = Vector::from_slice_clone(&["x".to_string(), "y".to_string(), "z".to_string()]);
        assert_eq!(c.as_slice(), &["x", "y", "z"]);
        std::mem::forget(c);
    }


    // ==== From & Into ====
    #[test]
    fn test_from_std_vec() {
        let v: Vec<u8> = vec![1, 2, 3, 4, 5];
        let v2: Vector<u8> = Vector::from(v);

        assert_eq!(v2.as_slice(), &[1, 2, 3, 4, 5]);

        // &str literal
        let v: Vec<&str> = vec!["x", "y", "z"];
        let v2: Vector<&str> = Vector::from(v);

        assert_eq!(v2.as_slice(), &["x", "y", "z"]);
    }

    #[test]
    fn test_into_std_vec() {
        let v: Vec<u8> = {
            let v2: Vector<u8> = Vector::from_slice_copy(&[1, 2, 3, 4, 5]);
            v2.into()
        };

        assert_eq!(v.as_slice(), &[1, 2, 3, 4, 5]);

        // &str literal
        let v: Vector<&str> = Vector::from_slice_clone(&["x", "y", "z"]);
        let v2: Vec<&str> = v.into();

        assert_eq!(v2.as_slice(), &["x", "y", "z"]);
    }

    // ==== Deref ====

    #[test]
    fn test_deref() {
        let v: Vector<i32> = Vector::new_init(5, &1);

        // assert_eq!(&*v, &[1, 1, 1, 1, 1]);
        // let deref: [i32] = *v; // as [i32; 10];
        // TODO What is [T] type? How can it be used without reference?

        assert_panic!({ (&*v)[100]; });
    }

    #[test]
    fn test_deref_iter() {
        let v: Vector<usize> = Vector::from_slice_copy(&[1, 2, 3, 4, 5]);

        // TODO How does it automatically gives iter()?
        let v2: Vec<usize> = v.iter()
            .map(|el| { el * el })
            // Requires FromIterator
            .collect();

        assert_eq!(v2.as_slice(), &[1, 4, 9, 16, 25]);
    }

    #[test]
    fn test_deref_join() {
        let mut v: Vector<String> = Vector::new(0);
        v.push("12".to_string());
        v.push("34".to_string());
        v.push("56".to_string());
        assert_eq!(v.join("|"), "12|34|56");
    }
}

