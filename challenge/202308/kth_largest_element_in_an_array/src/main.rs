fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    fn quick_select(nums: Vec<i32>, k: usize) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut lefts = vec![];
        let mut rights = vec![];

        let pivot = std::cmp::max(nums[0], nums[1]);
        let mut pivots = 0;

        for num in nums {
            if num > pivot {
                lefts.push(num);
            } else if num < pivot {
                rights.push(num);
            } else {
                pivots += 1;
            }
        }

        if k <= lefts.len() {
            quick_select(lefts, k)
        } else if lefts.len() + pivots < k {
            quick_select(rights, k - lefts.len() - pivots)
        } else {
            pivot
        }
    }

    quick_select(nums, k as usize)
}

fn main() {
    let nums = vec![3, 2, 1, 5, 6, 4];
    let ret = find_kth_largest(nums, 2);
    println!("ret={ret}");
}

#[test]
fn test_find_kth_largest() {
    {
        let nums = vec![3, 2, 1, 5, 6, 4];
        let ret = find_kth_largest(nums, 2);
        assert_eq!(ret, 5);
    }
    {
        let nums = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
        let ret = find_kth_largest(nums, 4);
        assert_eq!(ret, 4);
    }
}
