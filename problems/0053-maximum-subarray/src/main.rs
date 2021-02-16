fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut ret = nums[0];
    let mut sum = nums[0];
    for &n in nums.iter().skip(1) {
        if sum + n > n {
            sum += n;
            ret = std::cmp::max(ret, sum);
        } else {
            sum = n;
            ret = std::cmp::max(ret, n);
        }
    }

    ret
}

fn main() {
    println!(
        "max_sub_array([-2,1,-3,4,-1,2,1,-5,4])={}",
        max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
    );
}

#[test]
fn test_max_sub_array() {
    assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    assert_eq!(max_sub_array(vec![1]), 1);
    assert_eq!(max_sub_array(vec![0]), 0);
    assert_eq!(max_sub_array(vec![-1]), -1);
    assert_eq!(max_sub_array(vec![-100_000]), -100_000);
}
