fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut dp_asc = vec![1; len];
    let mut dp_dsc = vec![1; len];

    for i in 0..len {
        for j in (0..i).rev() {
            if nums[j] < nums[i] {
                dp_asc[i] = std::cmp::max(dp_asc[i], dp_asc[j] + 1);
            }
        }
    }

    for i in (0..len).rev() {
        for j in (i + 1)..len {
            if nums[i] > nums[j] {
                dp_dsc[i] = std::cmp::max(dp_dsc[i], dp_dsc[j] + 1);
            }
        }
    }

    let mut ret = len + 1;
    for i in 0..len {
        if dp_asc[i] > 1 && dp_dsc[i] > 1 {
            ret = std::cmp::min(ret, len + 1 - dp_asc[i] - dp_dsc[i]);
        }
    }

    ret as i32
}

fn main() {
    let nums = vec![2, 1, 1, 5, 6, 2, 3, 1];
    let ret = minimum_mountain_removals(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 3, 1];
        let ret = minimum_mountain_removals(nums);
        assert_eq!(ret, 0);
    }
    {
        let nums = vec![2, 1, 1, 5, 6, 2, 3, 1];
        let ret = minimum_mountain_removals(nums);
        assert_eq!(ret, 3);
    }
    {
        let nums = vec![9, 8, 1, 7, 6, 5, 4, 3, 2, 1];
        let ret = minimum_mountain_removals(nums);
        assert_eq!(ret, 2);
    }
}
