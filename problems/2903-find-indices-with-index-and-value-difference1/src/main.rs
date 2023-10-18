fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
    let len = nums.len();
    let index_diff = index_difference as usize;

    for i in 0..len {
        for j in (i + index_diff)..len {
            if (nums[i] - nums[j]).abs() >= value_difference {
                return vec![i as i32, j as i32];
            }
        }
    }

    vec![-1, -1]
}

fn main() {
    let nums = vec![5, 1, 4, 1];
    let ret = find_indices(nums, 2, 4);
    println!("ret={ret:?}");
}

#[test]
fn test_find_indices() {
    {
        let nums = vec![5, 1, 4, 1];
        let expected = vec![0, 3];
        let ret = find_indices(nums, 2, 4);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![2, 1];
        let expected = vec![0, 0];
        let ret = find_indices(nums, 0, 0);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![1, 2, 3];
        let expected = vec![-1, -1];
        let ret = find_indices(nums, 2, 4);
        assert_eq!(ret, expected);
    }
}
