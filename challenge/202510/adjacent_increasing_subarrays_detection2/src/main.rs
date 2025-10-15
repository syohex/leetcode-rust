fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
    let mut ret = 1;
    let mut prev = 1;
    let mut count = 1;

    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] {
            count += 1;
        } else {
            prev = count;
            count = 1;
        }

        ret = std::cmp::max(ret, std::cmp::min(prev, count));
        ret = std::cmp::max(ret, count / 2);
    }

    ret
}

fn main() {
    let nums = vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1];
    let ret = max_increasing_subarrays(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1];
        let ret = max_increasing_subarrays(nums);
        assert_eq!(ret, 3);
    }
    {
        let nums = vec![1, 2, 3, 4, 4, 4, 4, 5, 6, 7];
        let ret = max_increasing_subarrays(nums);
        assert_eq!(ret, 2);
    }
}
