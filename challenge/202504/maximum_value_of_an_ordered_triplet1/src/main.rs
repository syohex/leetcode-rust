fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
    let nums: Vec<_> = nums.into_iter().map(|n| n as i64).collect();
    let len = nums.len();

    let mut ret = 0;
    for i in 0..len {
        for j in (i + 1)..len {
            for k in (j + 1)..len {
                ret = std::cmp::max(ret, (nums[i] - nums[j]) * nums[k]);
            }
        }
    }

    ret
}

fn main() {
    let nums = vec![12, 6, 1, 2, 7];
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
