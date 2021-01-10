use std::collections::HashMap;

fn majority_element(nums: Vec<i32>) -> i32 {
    let mut h: HashMap<i32, i32> = HashMap::new();
    let mut max: i32 = 0;
    let mut ret = 0;
    for &n in nums.iter() {
        let mut tmp = 1;
        if let Some(count) = h.get_mut(&n) {
            *count += 1;
            tmp = *count;
        } else {
            h.insert(n, 1);
        }

        if tmp > max {
            ret = n;
            max = tmp;
        }
    }

    ret
}

fn main() {
    println!(
        "majority_element([3, 2, 3]) = {}",
        majority_element(vec![3, 2, 3])
    );
}

#[test]
fn test_majority_element() {
    assert_eq!(majority_element(vec![3, 2, 3]), 3);
    assert_eq!(majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
}
