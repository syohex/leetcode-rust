fn max_ascending_sum(nums: Vec<i32>) -> i32 {
    use std::cmp::max;

    let mut ret = nums[0];
    let mut sum = nums[0];
    for i in 1..nums.len() {
        if nums[i - 1] >= nums[i] {
            ret = max(ret, sum);
            sum = nums[i];
        } else {
            sum += nums[i];
        }
    }

    max(ret, sum)
}

fn main() {
    let ret = max_ascending_sum(vec![10,20,30,5,10,50]);
    println!("ret={}", ret);
}

#[test]
fn test_max_ascending_sum() {
    assert_eq!(max_ascending_sum(vec![10,20,30,5,10,50]), 65);
    assert_eq!(max_ascending_sum(vec![10,20,30,40,50]), 150);
    assert_eq!(max_ascending_sum(vec![12,17,15,13,10,11,12]), 33);
    assert_eq!(max_ascending_sum(vec![100,10,1]), 100);
}
