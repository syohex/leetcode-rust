fn count_good(nums: Vec<i32>, k: i32) -> i64 {
    use std::collections::HashMap;

    let len = nums.len();
    let mut ret = 0i64;
    let mut right = -1;
    let mut pairs = 0;
    let mut freq: HashMap<i32, i32> = HashMap::new();

    for &n in &nums {
        while pairs < k && (right + 1) < len as i32 {
            right += 1;

            if let Some(v) = freq.get_mut(&nums[right as usize]) {
                pairs += *v;
                *v += 1;
            } else {
                freq.insert(nums[right as usize], 1);
            }
        }

        if pairs >= k {
            ret += (len - right as usize) as i64;
        }

        if let Some(v) = freq.get_mut(&n) {
            *v -= 1;
            pairs -= *v;
        }
    }

    ret
}

fn main() {
    let nums = vec![3, 1, 4, 3, 2, 2, 4];
    let ret = count_good(nums, 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 1, 1, 1, 1];
        let ret = count_good(nums, 10);
        assert_eq!(ret, 1);
    }
    {
        let nums = vec![3, 1, 4, 3, 2, 2, 4];
        let ret = count_good(nums, 2);
        assert_eq!(ret, 4);
    }
}
