
#[cfg(test)]
mod tests {
    use containers::vector::Vector;
    use containers::vector::ConsecConstrucor;
    use all_asserts::{assert_false};
    use assert_panic::assert_panic;

    #[test]
    fn test_pop_till_empty() {
        let mut v = Vector::new(1, 3);
        assert_eq!(v.pop(), Some(1));
        assert_eq!(v.len, 2);
        assert_eq!(v.pop(), Some(1));
        assert_eq!(v.len, 1);
        assert_eq!(v.pop(), Some(1));
        assert_eq!(v.len, 0);
        assert_eq!(v.pop(), None);
    }

    #[test]
    fn test_insert() {
        let mut v = Vector::new(10, 5);
        assert_eq!(v.insert(6, 0), false);
        assert_eq!(v.insert(7, 0), false);
        assert_eq!(v.insert(3, 100), true);
        assert_eq!(v.insert(3, 100), true);
        assert_eq!(v.insert(0, 33), true);
    }

    #[test]
    fn test_insert_out_of_bounds() {
        let mut v = Vector::new(10, 5);
        assert_false!(v.insert(v.len, 100));
    }

    #[test]
    fn test_push_multiple() {
        let mut v = Vector::new(10, 5);

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

    #[test]
    fn test_push_empty() {
        let mut v = Vector::new(0, 0);
        assert_eq!(v.push(1), true);
    }

    #[test]
    fn test_as_slice() {
        let mut v = Vector::new(1, 5);
        assert_eq!(v.as_slice(), &[1, 1, 1, 1, 1]);

        v.push(10);
        assert_eq!(v.as_slice(), &[1, 1, 1, 1, 1, 10]);

        v.push(10);
        assert_eq!(v.as_slice(), &[1, 1, 1, 1, 1, 10, 10]);

        v.insert(1, 3);
        assert_eq!(v.as_slice(), &[1, 3, 1, 1, 1, 1, 10, 10]);

        assert_eq!(v.pop(), Some(10));
        assert_eq!(v.as_slice(), &[1, 3, 1, 1, 1, 1, 10]);

        v[2] = 100;
        assert_eq!(v.as_slice(), &[1, 3, 100, 1, 1, 1, 10]);
    }

    #[test]
    fn test_as_ptr() {
        let mut v = Vector::new(1, 5);
        v[0] = 10;
        v[1] = 20;
        v[2] = 30;
        v[3] = 40;
        v[4] = 50;
        assert_eq!(v.as_slice(), &[10, 20, 30, 40, 50]);

        let ptr: *const u8 = v.as_ptr();
        assert_eq!(v.len_bytes(), 5);
        unsafe {
            assert_eq!(*ptr, 10);
            assert_eq!(*ptr.add(size_of::<u8>()), 20);
            assert_eq!(*ptr.add(size_of::<u8>() * 2), 30);
            assert_eq!(*ptr.add(size_of::<u8>() * 3), 40);
            assert_eq!(*ptr.add(size_of::<u8>() * 4), 50);
        }
    }

    #[test]
    fn test_as_mut_ptr() {
        let mut v = Vector::new(1, 5);
        v[0] = 10;
        v[1] = 20;
        v[2] = 30;
        v[3] = 40;
        v[4] = 50;
        assert_eq!(v.as_slice(), &[10, 20, 30, 40, 50]);

        let ptr: *mut u8 = v.as_mut_ptr();
        unsafe {
            assert_eq!(*ptr, 10);
            assert_eq!(*ptr.add(1), 20);
            assert_eq!(*ptr.add(2), 30);
            assert_eq!(*ptr.add(3), 40);
            assert_eq!(*ptr.add(4), 50);
        }

        unsafe {
            *ptr.add(1) = 25;
        }
        assert_eq!(v.as_slice(), &[10, 25, 30, 40, 50]);

        unsafe {
            *ptr.add(4) = 55;
        }
        assert_eq!(v.as_slice(), &[10, 25, 30, 40, 55]);
    }

    #[test]
    fn test_consec() {
        let mut v: Vector<usize> = Vector::consec(10);
        assert_eq!(v.as_slice(), &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_deref() {
        let mut v: Vector<i32> = Vector::new(1, 5);

        // assert_eq!(&*v, &[1, 1, 1, 1, 1]);
        // let deref: [i32] = *v; // as [i32; 10];
        // TODO What is [T] type? How can it be used without reference?

        // TODO Can C++ check for bounds in such way
        assert_panic!({ (&*v)[100]; });
    }

    #[test]
    fn test_deref_iter() {
        let mut v: Vector<usize> = Vector::consec(5);

        // How does it automatically gives iter()?
        let v2: Vec<usize> = v.iter()
            .map(|el| { el * el })
            // Requires FromIterator
            .collect();

        assert_eq!(v2.as_slice(), &[1, 4, 9, 16, 25]);
    }
}
