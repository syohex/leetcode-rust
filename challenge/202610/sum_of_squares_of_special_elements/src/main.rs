fn sum_of_squares(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    nums.into_iter().enumerate().fold(0, |acc, (i, n)| {
        if len.is_multiple_of(i + 1) {
            acc + n * n
        } else {
            acc
        }
    })
}

fn main() {
    let ret = sum_of_squares(vec![2, 7, 1, 19, 18, 3]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(sum_of_squares(vec![1, 2, 3, 4]), 21);
    assert_eq!(sum_of_squares(vec![2, 7, 1, 19, 18, 3]), 63);
}
