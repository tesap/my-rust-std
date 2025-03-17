
#[cfg(test)]
mod tests {
    use containers::bytes2 as my;
    use containers::bytes2::DebugBytes;

    fn sample_from_bytes() -> my::Bytes {
        my::Bytes::from_bytes(&[1, 2, 3, 4, 5])
    }

    fn sample_from_int() -> my::Bytes {
        let x: i128 = 21542142465;
        my::Bytes::from_int(&x)
    }

    #[test]
    fn test_from_bytes() {
        let mut b: my::Bytes = sample_from_bytes();
        b.print();

        assert_eq!(b.as_slice(), &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_from_int() {
        let b: my::Bytes = sample_from_int();

        b.print();
    }

    #[test]
    fn test_compare() {
        let b1 = sample_from_bytes();
        let b2 = sample_from_int();

        assert_eq!(b1.to_int128(), b2.to_int128());
        // TODO Fails
        // assert_eq!(b1.as_slice(), b2.as_slice());
        // TODO Add PartialEq for Vector
        //assert_eq!(b1.to_bin().as_slice(), b2.to_bin().as_slice());
    }
}
