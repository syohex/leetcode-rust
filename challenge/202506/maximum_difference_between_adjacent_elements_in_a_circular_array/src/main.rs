fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
    let mut ret = 0;
    for i in 0..nums.len() {
        ret = std::cmp::max(ret, (nums[i] - nums[(i + 1) % nums.len()]).abs());
    }

    ret
}

fn main() {
    let nums = vec![-5, -10, -5];
    let ret = max_adjacent_distance(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 2, 4];
        let ret = max_adjacent_distance(nums);
        assert_eq!(ret, 3);
    }
    {
        let nums = vec![-5, -10, -5];
        let ret = max_adjacent_distance(nums);
        assert_eq!(ret, 5);
    }
}
