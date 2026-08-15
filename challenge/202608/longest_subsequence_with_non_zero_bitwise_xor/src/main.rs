fn longest_subsequence(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let all = nums.iter().fold(0, |acc, &n| acc ^ n);
    if all != 0 {
        len as i32
    } else {
        if nums.iter().all(|&n| n == 0) {
            0
        } else {
            (len - 1) as i32
        }
    }
}

fn main() {
    let ret = longest_subsequence(vec![1, 2, 3]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(longest_subsequence(vec![7, 6, 1, 9]), 4);
    assert_eq!(longest_subsequence(vec![1, 2, 3]), 2);
    assert_eq!(longest_subsequence(vec![2, 3, 4]), 3);
}
