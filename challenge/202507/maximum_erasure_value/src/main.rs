fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut ret = 0;
    let mut h: HashMap<i32, i32> = HashMap::new();
    let mut left = 0;
    let mut sum = 0;

    for i in 0..nums.len() {
        *h.entry(nums[i]).or_insert(0) += 1;
        sum += nums[i];

        loop {
            let n = h.get(&nums[i]).unwrap();
            if *n == 1 {
                break;
            }

            sum -= nums[left];
            if let Some(v) = h.get_mut(&nums[left]) {
                *v -= 1;
            }
            left += 1;
        }

        ret = std::cmp::max(ret, sum);
    }

    ret
}

fn main() {
    let nums = vec![4, 2, 4, 5, 6];
    let ret = maximum_unique_subarray(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![4, 2, 4, 5, 6];
        let ret = maximum_unique_subarray(nums);
        assert_eq!(ret, 17);
    }
    {
        let nums = vec![5, 2, 1, 2, 5, 2, 1, 2, 5];
        let ret = maximum_unique_subarray(nums);
        assert_eq!(ret, 8);
    }
}
