use std::collections::HashMap;

fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let mut h: HashMap<i32, i32> = HashMap::new();
    for &n in &nums {
        *h.entry(n).or_insert(0) += 1;
    }

    let mut repeated = 0;
    let mut missed = 0;

    for i in 1..=nums.len() {
        let n = i as i32;
        if let Some(&count) = h.get(&n) {
            if count == 2 {
                repeated = n;
            }
        } else {
            missed = n;
        }
    }

    vec![repeated, missed]
}

fn main() {
    let ret = find_error_nums(vec![1, 2, 2, 4]);
    println!("ret={:?}", ret);
}

#[test]
fn test_find_error_nums() {
    assert_eq!(find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
    assert_eq!(find_error_nums(vec![1, 1]), vec![1, 2]);
}
