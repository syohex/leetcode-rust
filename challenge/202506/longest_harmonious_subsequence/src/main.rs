fn find_lfs(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let h = nums.into_iter().fold(HashMap::new(), |mut acc, n| {
        *acc.entry(n).or_insert(0) += 1;
        acc
    });

    let mut ret = 0;
    for (&k, &v) in &h {
        if let Some(&v2) = h.get(&(k - 1)) {
            ret = std::cmp::max(ret, v + v2);
        }

        if let Some(&v2) = h.get(&(k + 1)) {
            ret = std::cmp::max(ret, v + v2);
        }
    }

    ret
}

fn main() {
    let nums = vec![1, 3, 2, 2, 5, 2, 3, 7];
    let ret = find_lfs(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 3, 2, 2, 5, 2, 3, 7];
        let ret = find_lfs(nums);
        assert_eq!(ret, 5);
    }
    {
        let nums = vec![1, 2, 3, 4];
        let ret = find_lfs(nums);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![1, 1, 1, 1];
        let ret = find_lfs(nums);
        assert_eq!(ret, 0);
    }
}
