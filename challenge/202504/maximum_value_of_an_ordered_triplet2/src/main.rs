fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
    let nums: Vec<_> = nums.into_iter().map(|n| n as i64).collect();
    let len = nums.len();

    let mut prefixes = vec![0; len];
    prefixes[0] = nums[0];
    for i in 1..len {
        prefixes[i] = std::cmp::max(prefixes[i - 1], nums[i]);
    }

    let mut postfixes = vec![0; len];
    postfixes[len - 1] = nums[len - 1];

    for i in (0..(len - 1)).rev() {
        postfixes[i] = std::cmp::max(postfixes[i + 1], nums[i]);
    }

    let mut ret = 0;
    for i in 1..(len - 1) {
        ret = std::cmp::max(ret, (prefixes[i-1] - nums[i]) * postfixes[i + 1]);
    }

    ret
}

fn main() {
    let nums = vec![1, 10, 3, 4, 19];
    let ret = maximum_triplet_value(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![12, 6, 1, 2, 7];
        let ret = maximum_triplet_value(nums);
        assert_eq!(ret, 77);
    }
    {
        let nums = vec![1, 10, 3, 4, 19];
        let ret = maximum_triplet_value(nums);
        assert_eq!(ret, 133);
    }
    {
        let nums = vec![1, 2, 3];
        let ret = maximum_triplet_value(nums);
        assert_eq!(ret, 0);
    }
}
