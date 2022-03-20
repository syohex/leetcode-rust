fn divide_array(nums: Vec<i32>) -> bool {
    use std::collections::HashMap;

    let mut h = HashMap::new();
    for num in nums {
        *h.entry(num).or_insert(0) += 1;
    }

    h.values().all(|&v| v % 2 == 0)
}

fn main() {
    println!("ret={}", divide_array(vec![3, 2, 3, 2, 2, 2]));
}

#[test]
fn test_divide_array() {
    {
        let nums = vec![3, 2, 3, 2, 2, 2];
        assert!(divide_array(nums));
    }
    {
        let nums = vec![3, 2, 3, 2, 2];
        assert!(!divide_array(nums));
    }
    {
        let nums = vec![1, 2, 3, 4];
        assert!(!divide_array(nums));
    }
    {
        let nums = vec![1];
        assert!(!divide_array(nums));
    }
}
