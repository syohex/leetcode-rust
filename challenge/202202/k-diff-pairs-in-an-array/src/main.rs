fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashMap;

    let mut h = HashMap::new();
    for n in &nums {
        *h.entry(*n).or_insert(0) += 1;
    }

    h.keys().into_iter().fold(0, |acc, n| {
        if let Some(&v) = h.get(&(n - k)) {
            if k == 0 {
                if v >= 2 {
                    acc + 1
                } else {
                    acc
                }
            } else {
                acc + 1
            }
        } else {
            acc
        }
    })
}

fn main() {
    let nums = vec![3, 1, 4, 1, 5];
    let ret = find_pairs(nums, 2);
    println!("ret={ret}");
}

#[test]
fn test_find_pairs() {
    {
        let nums = vec![3, 1, 4, 1, 5];
        assert_eq!(find_pairs(nums, 2), 2);
    }
    {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(find_pairs(nums, 1), 4);
    }
    {
        let nums = vec![1, 3, 1, 5, 4];
        assert_eq!(find_pairs(nums, 0), 1);
    }
}
