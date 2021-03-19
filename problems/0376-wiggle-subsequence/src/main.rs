fn wiggle_max_length(nums: Vec<i32>) -> i32 {
    let mut count = 1;
    let mut prev_diff = 0;
    for i in 1..nums.len() {
        let diff = nums[i] - nums[i - 1];
        if (diff > 0 && prev_diff <= 0) || (diff < 0 && prev_diff >= 0) {
            count += 1;
            prev_diff = diff;
        }
    }

    count
}

fn main() {
    let ret = wiggle_max_length(vec![1, 7, 4, 9, 2, 5]);
    println!("ret={}", ret);
}

#[test]
fn test_wiggle_max_length() {
    assert_eq!(wiggle_max_length(vec![1, 7, 4, 9, 2, 5]), 6);
    assert_eq!(
        wiggle_max_length(vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8]),
        7
    );
    assert_eq!(wiggle_max_length(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 2);
}
