fn max_k_distinct(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::HashSet;

    let k = k as usize;
    let mut nums = nums;
    nums.sort_unstable();

    let mut ret = vec![];
    let mut s = HashSet::new();
    for n in nums.into_iter().rev() {
        if s.contains(&n) {
            continue;
        }

        ret.push(n);
        s.insert(n);

        if s.len() >= k {
            break;
        }
    }

    ret
}

fn main() {
    let nums = vec![84, 93, 100, 77, 93];
    let ret = max_k_distinct(nums, 3);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let nums = vec![84, 93, 100, 77, 90];
        let ret = max_k_distinct(nums, 3);
        assert_eq!(ret, vec![100, 93, 90]);
    }
    {
        let nums = vec![84, 93, 100, 77, 93];
        let ret = max_k_distinct(nums, 3);
        assert_eq!(ret, vec![100, 93, 84]);
    }
    {
        let nums = vec![1, 1, 1, 2, 2, 2];
        let ret = max_k_distinct(nums, 6);
        assert_eq!(ret, vec![2, 1]);
    }
}
