fn alternating_subarray(nums: Vec<i32>) -> i32 {
    let len = nums.len();

    let mut ret = -1;
    for i in 0..len {
        let mut diff = 1;
        let mut n = 1;
        for j in (i + 1)..len {
            if nums[j] - nums[j - 1] == diff {
                n += 1;
            } else {
                break;
            }

            diff = if diff == 1 { -1 } else { 1 }
        }

        if n != 1 {
            ret = std::cmp::max(ret, n);
        }
    }

    ret
}

fn main() {
    let nums = vec![2, 3, 4, 3, 4];
    let ret = alternating_subarray(nums);
    println!("ret={ret}");
}

#[test]
fn test_alternating_subarray() {
    {
        let nums = vec![2, 3, 4, 3, 4];
        let ret = alternating_subarray(nums);
        assert_eq!(ret, 4);
    }
    {
        let nums = vec![4, 5, 6];
        let ret = alternating_subarray(nums);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![1, 1, 1];
        let ret = alternating_subarray(nums);
        assert_eq!(ret, -1);
    }
}
