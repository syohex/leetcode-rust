fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut ret = 0;
    let mut start = 0;

    let mut acc = vec![0; nums.len() + 1];
    let mut h = HashMap::new();
    for (i, n) in nums.into_iter().enumerate() {
        acc[i + 1] = acc[i] + n;

        if let Some(&p) = h.get(&n) {
            start = std::cmp::max(start, p + 1);
        }

        h.insert(n, i);
        ret = std::cmp::max(ret, acc[i + 1] - acc[start]);
    }

    ret
}

fn main() {
    let nums = vec![4, 2, 4, 5, 6];
    let ret = maximum_unique_subarray(nums);
    println!("ret={ret}");
}

#[test]
fn test_maximum_unique_subarray() {
    {
        let nums = vec![4, 2, 4, 5, 6];
        let ret = maximum_unique_subarray(nums);
        assert_eq!(ret, 17);
    }
    {
        let nums = vec![5, 2, 1, 2, 5, 2, 1, 2, 5];
        let ret = maximum_unique_subarray(nums);
        assert_eq!(ret, 8);
    }
    {
        let nums = vec![1];
        let ret = maximum_unique_subarray(nums);
        assert_eq!(ret, 1);
    }
    {
        let nums = vec![558, 508, 782, 32, 187, 103, 370, 607, 619, 267, 984, 10];
        let ret = maximum_unique_subarray(nums);
        assert_eq!(ret, 5027);
    }
    {
        let nums = vec![
            187, 470, 25, 436, 538, 809, 441, 167, 477, 110, 275, 133, 666, 345, 411, 459, 490,
            266, 987, 965, 429, 166, 809, 340, 467, 318, 125, 165, 809, 610, 31, 585, 970, 306, 42,
            189, 169, 743, 78, 810, 70, 382, 367, 490, 787, 670, 476, 278, 775, 673, 299, 19, 893,
            817, 971, 458, 409, 886, 434,
        ];
        let ret = maximum_unique_subarray(nums);
        assert_eq!(ret, 16911);
    }
}
