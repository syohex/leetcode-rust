fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
    let key_indices: Vec<i32> = nums
        .iter()
        .enumerate()
        .filter_map(|(i, n)| if *n == key { Some(i as i32) } else { None })
        .collect();

    (0..nums.len() as i32)
        .filter(|i| key_indices.iter().any(|&j| (i - j).abs() <= k))
        .collect()
}

fn main() {
    let nums = vec![3, 4, 9, 1, 3, 9, 5];
    let ret = find_k_distant_indices(nums, 9, 1);
    println!("ret={:?}", ret);
}

#[test]
fn test_find_k_distance_indices() {
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
