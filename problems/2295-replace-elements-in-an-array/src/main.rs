fn array_change(nums: Vec<i32>, operations: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::HashMap;

    let mut h = HashMap::new();
    for (i, n) in nums.iter().enumerate() {
        h.insert(*n, i);
    }

    let mut nums = nums;
    for op in operations {
        if let Some(&i) = h.get(&op[0]) {
            nums[i] = op[1];
            h.remove(&op[0]);
            h.insert(op[1], i);
        }
    }

    nums
}

fn main() {
    let nums = vec![1, 2, 4, 6];
    let operations = vec![vec![1, 3], vec![4, 7], vec![6, 1]];
    let ret = array_change(nums, operations);
    println!("ret={:?}", ret);
}

#[test]
fn test_array_change() {
    {
        let nums = vec![1, 2, 4, 6];
        let operations = vec![vec![1, 3], vec![4, 7], vec![6, 1]];
        let expected = vec![3, 2, 7, 1];
        let ret = array_change(nums, operations);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![1, 2];
        let operations = vec![vec![1, 3], vec![2, 1], vec![3, 2]];
        let expected = vec![2, 1];
        let ret = array_change(nums, operations);
        assert_eq!(ret, expected);
    }
}
