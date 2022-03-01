fn count_bits(n: i32) -> Vec<i32> {
    (0..=n).into_iter().map(|m| m.count_ones() as i32).collect()
}
fn main() {
    let ret = count_bits(5);
    println!("ret={:?}", ret);
}

#[test]
fn test_count_bits() {
    {
        let expected = vec![0, 1, 1];
        assert_eq!(count_bits(2), expected);
    }
    {
        let expected = vec![0, 1, 1, 2, 1, 2];
        assert_eq!(count_bits(5), expected);
    }
}
