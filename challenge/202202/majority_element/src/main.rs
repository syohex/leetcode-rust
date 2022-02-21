fn majority_element(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut h = HashMap::new();
    let threshold = (nums.len() / 2) + 1;
    for &num in &nums {
        if let Some(v) = h.get_mut(&num) {
            *v += 1;
            if *v as usize >= threshold {
                return num;
            }
        } else {
            h.insert(num, 1);
        }
    }

    nums[0]
}

fn main() {
    let nums = vec![2, 2, 1, 1, 1, 2, 2];
    let ret = majority_element(nums);
    println!("ret={ret}");
}

#[test]
fn test_majority_element() {
    {
        let nums = vec![3, 2, 3];
        assert_eq!(majority_element(nums), 3);
    }
    {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        assert_eq!(majority_element(nums), 2);
    }
    {
        let nums = vec![1];
        assert_eq!(majority_element(nums), 1);
    }
}
