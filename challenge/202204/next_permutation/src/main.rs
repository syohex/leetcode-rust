fn next_permutation(nums: &mut Vec<i32>) {
    fn reverse(nums: &mut [i32], start: usize, end: usize) {
        let mut start = start as i32;
        let mut end = end as i32;

        while start < end {
            nums.swap(start as usize, end as usize);
            start += 1;
            end -= 1;
        }
    }

    let len = nums.len();
    let mut i = len as i32 - 2;
    while i >= 0 && nums[i as usize] >= nums[i as usize + 1] {
        i -= 1;
    }

    if i < 0 {
        reverse(nums, 0, len - 1);
        return;
    }

    let mut j = nums.len() as i32 - 1;
    while j >= 0 && nums[i as usize] >= nums[j as usize] {
        j -= 1;
    }

    nums.swap(i as usize, j as usize);
    reverse(nums, i as usize + 1, len - 1);
}

fn main() {
    let mut nums = vec![1, 2, 3];
    next_permutation(&mut nums);
    println!("ret={:?}", nums);
}

#[test]
fn test_next_permutation() {
    {
        let mut nums = vec![1, 2, 3];
        let expected = vec![1, 3, 2];
        next_permutation(&mut nums);
        assert_eq!(nums, expected);
    }
    {
        let mut nums = vec![3, 2, 1];
        let expected = vec![1, 2, 3];
        next_permutation(&mut nums);
        assert_eq!(nums, expected);
    }
    {
        let mut nums = vec![1, 1, 5];
        let expected = vec![1, 5, 1];
        next_permutation(&mut nums);
        assert_eq!(nums, expected);
    }
    {
        let mut nums = vec![1, 5, 1];
        let expected = vec![5, 1, 1];
        next_permutation(&mut nums);
        assert_eq!(nums, expected);
    }
}
