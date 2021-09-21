fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut ret = 0;
    let mut count = 0;
    for (i, &num) in nums.iter().enumerate() {
        if num == 1 {
            count += 1;
        }

        if i == nums.len() - 1 || num == 0 {
            ret = std::cmp::max(ret, count);
            count = 0;
        }
    }
    ret
}

fn main() {
    let nums = vec![1, 1, 0, 1, 1, 1];
    let ret = find_max_consecutive_ones(nums);
    println!("ret={}", ret);
}

#[test]
fn test_find_max_consecutive_ones() {
    {
        let nums = vec![1, 1, 0, 1, 1, 1];
        assert_eq!(find_max_consecutive_ones(nums), 3);
    }
    {
        let nums = vec![1, 0, 1, 1, 0, 1];
        assert_eq!(find_max_consecutive_ones(nums), 2);
    }
}
