fn min_merged_integer(nums: Vec<i32>) -> i32 {
    nums.windows(2)
        .map(|w| {
            if w[1] >= 10 {
                w[0] * 100 + w[1]
            } else {
                w[0] * 10 + w[1]
            }
        })
        .min()
        .unwrap()
}

fn main() {
    let ret = min_merged_integer(vec![23, 55, 42, 21]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(min_merged_integer(vec![23, 55, 42, 21]), 2355);
    assert_eq!(min_merged_integer(vec![34, 7, 11, 11]), 347);
    assert_eq!(min_merged_integer(vec![9, 20]), 920);
}
