
macro_rules! test_parametrized_sample {
    ($func_name:ident, $test_name:ident, $sample:expr) => {
        #[test]
        fn $test_name() {
            return $func_name($sample);
        }
    }
}

#[cfg(test)]
mod tests {
    use tesap_std::{Bins, Bytes, Hex, Bin, DebugBytes};
    use tesap_std::Vector;

    struct TestSample<'a>{
        bytes: &'a [u8],
        int: i128,
        bins: Bins,
        hex: Hex,
    }

    // Simple
    fn s1() -> TestSample<'static> {
        TestSample {
            bytes: &[1, 2, 3, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            int: 21542142465,
            bins: Bins(Vector::from_slice_clone(&[
                "00000001".to_string(),
                "00000010".to_string(),
                "00000011".to_string(),
                "00000100".to_string(),
                "00000101".to_string(),
                "00000000".to_string(),
                "00000000".to_string(),
                "00000000".to_string(),
                "00000000".to_string(),
                "00000000".to_string(),
                "00000000".to_string(),
                "00000000".to_string(),
                "00000000".to_string(),
                "00000000".to_string(),
                "00000000".to_string(),
                "00000000".to_string(),
            ])),
            hex: Hex("01020304050000000000000000000000".to_string()),
        }
    }

    // TODO Add more samples

    fn sample_from_bytes(ts: &TestSample) -> Bytes {
        Bytes::from_bytes(&ts.bytes)
    }

    fn sample_from_int(ts: &TestSample) -> Bytes {
        Bytes::from_int(&ts.int)
    }

    fn sample_from_bin(ts: &TestSample) -> Bytes {
        Bytes::from_bins(&ts.bins)
    }

    fn sample_from_hex(ts: &TestSample) -> Bytes {
        Bytes::from_hex(&ts.hex)
    }

    fn compare_bytes(b1: &Bytes, b2: &Bytes) {
        assert_eq!(b1.as_slice(), b2.as_slice());
        assert_eq!(b1.to_int128(), b2.to_int128());
        // TODO Add PartialEq for Vector
        assert_eq!(b1.to_bin().0.as_slice(), b2.to_bin().0.as_slice());
        // TODO Add PartialEq for Hex
        assert_eq!(b1.to_hex().0, b2.to_hex().0);

    }

    fn test_compare_pairwise(ts: &TestSample) {
        let b1 = sample_from_bytes(ts);
        let b2 = sample_from_int(ts);
        let b3 = sample_from_bin(ts);
        let b4 = sample_from_hex(ts);

        compare_bytes(&b1, &b2);
        compare_bytes(&b1, &b3);
        compare_bytes(&b1, &b4);
        compare_bytes(&b2, &b3);
        compare_bytes(&b2, &b4);
        compare_bytes(&b3, &b4);
    }

    // === TESTS ===
    test_parametrized_sample!(test_compare_pairwise, test_compare_pairwise_s1, &s1());

    #[test]
    fn test_print() {
        let b1 = sample_from_bytes(&s1());
        let b2 = sample_from_int(&s1());
        let b3 = sample_from_bin(&s1());
        let b4 = sample_from_hex(&s1());

        b1.print();
        b2.print();
        b3.print();
        b4.print();
    }

    #[test]
    fn test_from_vec() {
        let bs: Bytes = {
            let v: Vector<u8> = Vector::from_slice_copy(&[1, 2, 3, 4, 5]);
            Bytes::from(v)
        };

        bs.print();
        assert_eq!(bs.as_slice(), &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_into_vec() {
        let v: Vector<u8> = {
            let bs: Bytes = Bytes::from_bytes(&[1, 2, 3, 4, 5]);
            bs.into()
        };

        assert_eq!(v.as_slice(), &[1, 2, 3, 4, 5]);
    }


    #[test]
    fn test_from_std_vec() {
        let bs: Bytes = {
            let v: Vec<u8> = vec![1, 2, 3, 4, 5];
            Bytes::from(v)
        };

        bs.print();
        assert_eq!(bs.as_slice(), &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_into_std_vec() {
        let v: Vec<u8> = {
            let bs: Bytes = Bytes::from_bytes(&[1, 2, 3, 4, 5]);
            bs.into()
        };

        assert_eq!(v.as_slice(), &[1, 2, 3, 4, 5]);
    }
}
