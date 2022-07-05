fn longest_consective(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    let mut s = HashSet::new();
    for &num in &nums {
        s.insert(num);
    }

    let mut ret = 0;

    for &n in s.iter() {
        let prev = n - 1;
        if s.contains(&prev) {
            continue;
        }

        let mut len = 1;
        let mut next = n + 1;
        while s.contains(&next) {
            next += 1;
            len += 1;
        }

        ret = std::cmp::max(ret, len);
    }

    ret
}

fn main() {
    let nums = vec![100, 4, 200, 1, 3, 2];
    let ret = longest_consective(nums);
    println!("ret={ret}");
}

#[test]
fn test_longest_consecuive_sequence() {
    {
        let nums = vec![100, 4, 200, 1, 3, 2];
        let ret = longest_consective(nums);
        assert_eq!(ret, 4);
    }
    {
        let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        let ret = longest_consective(nums);
        assert_eq!(ret, 9);
    }
}
