
#[cfg(test)]
mod tests {
    use containers::bytes2 as my;
    use all_asserts::{assert_false};

    #[test]
    fn test_abc() {
        let b: my::Bytes::<true> = my::Bytes::from_bytes(&[1, 2, 3, 4, 5]);
        println!("B: {:?}", b);
        println!("I128: {:?}", b.to_int128());
        println!("BIN: {:?}", b.to_bin());
    }
}
