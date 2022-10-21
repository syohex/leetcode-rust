fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    use std::collections::HashMap;

    let k = k as usize;
    let mut h: HashMap<i32, usize> = HashMap::new();
    for (i, num) in nums.into_iter().enumerate() {
        if let Some(last) = h.get(&num) {
            if i - *last <= k {
                return true;
            }
        }

        h.insert(num, i);
    }

    false
}

fn main() {
    let nums = vec![1, 2, 3, 1];
    let ret = contains_nearby_duplicate(nums, 3);
    println!("ret={ret}");
}

#[test]
fn test_contains_nearby_duplicate() {
    {
        let nums = vec![1, 2, 3, 1];
        let ret = contains_nearby_duplicate(nums, 3);
        assert!(ret);
    }
    {
        let nums = vec![1, 0, 1, 1];
        let ret = contains_nearby_duplicate(nums, 1);
        assert!(ret);
    }
    {
        let nums = vec![1, 2, 3, 1, 2, 3];
        let ret = contains_nearby_duplicate(nums, 2);
        assert!(!ret);
    }
}
