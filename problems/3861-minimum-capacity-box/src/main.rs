fn minimum_index(capacity: Vec<i32>, item_size: i32) -> i32 {
    let mut ret = -1;
    let mut min_cap = i32::MAX;

    for (i, n) in capacity.into_iter().enumerate() {
        if n >= item_size && n < min_cap {
            min_cap = n;
            ret = i as i32;
        }
    }

    ret
}
fn main() {
    let ret = minimum_index(vec![1, 5, 3, 7], 3);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(minimum_index(vec![1, 5, 3, 7], 3), 2);
    assert_eq!(minimum_index(vec![3, 5, 4, 3], 2), 0);
    assert_eq!(minimum_index(vec![4], 5), -1);
}
