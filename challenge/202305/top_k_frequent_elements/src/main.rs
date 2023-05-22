fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut h = HashMap::new();
    for num in nums {
        *h.entry(num).or_insert(0) += 1;
    }

    let mut v = Vec::from_iter(&h);
    v.sort_unstable_by(
        |(k1, v1), (k2, v2)| {
            if v1 == v2 {
                k1.cmp(k2)
            } else {
                v2.cmp(v1)
            }
        },
    );

    v.into_iter().take(k as usize).map(|(k, _)| *k).collect()
}

fn main() {
    let nums = vec![1, 1, 1, 2, 2, 3];
    let ret = top_k_frequent(nums, 2);
    println!("ret={ret:?}");
}

#[test]
fn test_top_k_frequent() {
    {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let expected = vec![1, 2];
        let ret = top_k_frequent(nums, 2);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![1];
        let expected = vec![1];
        let ret = top_k_frequent(nums, 1);
        assert_eq!(ret, expected);
    }
}
