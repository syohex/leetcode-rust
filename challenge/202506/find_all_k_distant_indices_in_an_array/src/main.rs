fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
    let mut ret = vec![];
    let len = nums.len();

    for i in 0..len {
        let min = std::cmp::max(i as i32 - k, 0) as usize;
        let max = std::cmp::min(i + k as usize, len - 1);

        for j in min..=max {
            if nums[j] == key {
                ret.push(i as i32);
                break;
            }
        }
    }

    ret
}

fn main() {
    let nums = vec![3, 4, 9, 1, 3, 9, 5];
    let ret = find_k_distant_indices(nums, 9, 1);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let nums = vec![3, 4, 9, 1, 3, 9, 5];
        let expected = vec![1, 2, 3, 4, 5, 6];
        let ret = find_k_distant_indices(nums, 9, 1);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![2, 2, 2, 2, 2];
        let expected = vec![0, 1, 2, 3, 4];
        let ret = find_k_distant_indices(nums, 2, 2);
        assert_eq!(ret, expected);
    }
}
