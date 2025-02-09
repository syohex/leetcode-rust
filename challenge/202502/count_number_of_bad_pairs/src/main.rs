fn count_bad_pairs(nums: Vec<i32>) -> i64 {
    use std::collections::HashMap;

    let mut h = HashMap::new();
    for (i, &n) in nums.iter().enumerate() {
        let diff = n - i as i32;
        *h.entry(diff).or_insert(0) += 1;
    }

    let mut rest = nums.len() as i64;
    let mut ret = 0;
    for (i, n) in nums.into_iter().enumerate() {
        let diff = n - i as i32;

        if let Some(v) = h.get_mut(&diff) {
            ret += rest - *v;
            *v -= 1;
        }

        rest -= 1;
    }

    ret
}

fn main() {
    let nums = vec![4, 1, 3, 3];
    let ret = count_bad_pairs(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![4, 1, 3, 3];
        let ret = count_bad_pairs(nums);
        assert_eq!(ret, 5);
    }
    {
        let nums = vec![1, 2, 3, 4, 5];
        let ret = count_bad_pairs(nums);
        assert_eq!(ret, 0);
    }
}
