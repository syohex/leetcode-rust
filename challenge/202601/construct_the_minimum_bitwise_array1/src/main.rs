fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
    nums.into_iter()
        .map(|n| {
            if n % 2 == 0 {
                -1
            } else {
                for i in 0..n {
                    if (i | (i + 1)) == n {
                        return i;
                    }
                }

                -1
            }
        })
        .collect()
}

fn main() {
    let ret = min_bitwise_array(vec![2, 3, 5, 7]);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    assert_eq!(min_bitwise_array(vec![2, 3, 5, 7]), vec![-1, 1, 4, 3]);
    assert_eq!(min_bitwise_array(vec![11, 13, 31]), vec![9, 12, 15]);
}
