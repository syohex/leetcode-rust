fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
    let mut ret = 1;
    let mut count = 1;
    let mut is_increasing = true;

    for (i, &num) in nums.iter().enumerate().skip(1) {
        if count == 1 {
            if num > nums[i - 1] {
                count += 1;
                is_increasing = true;
            }
            if num < nums[i - 1] {
                count += 1;
                is_increasing = false;
            }
        } else {
            if nums[i - 1] == num {
                count = 1;
            } else if nums[i - 1] < num {
                if is_increasing {
                    count += 1;
                } else {
                    count = 2;
                    is_increasing = true;
                }
            } else {
                if is_increasing {
                    count = 2;
                    is_increasing = false;
                } else {
                    count += 1;
                }
            }
        }

        ret = std::cmp::max(ret, count);
    }

    ret
}

fn main() {
    let nums = vec![1, 4, 3, 3, 2];
    let ret = longest_monotonic_subarray(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 4, 3, 3, 2];
        let ret = longest_monotonic_subarray(nums);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![3, 3, 3, 3];
        let ret = longest_monotonic_subarray(nums);
        assert_eq!(ret, 1);
    }
    {
        let nums = vec![3, 2, 1];
        let ret = longest_monotonic_subarray(nums);
        assert_eq!(ret, 3);
    }
    {
        let nums = vec![1,9,7,1];
        let ret = longest_monotonic_subarray(nums);
        assert_eq!(ret, 3);
    }
}
