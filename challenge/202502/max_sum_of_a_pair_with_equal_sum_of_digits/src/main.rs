fn maximum_sum(nums: Vec<i32>) -> i32 {
    use std::cmp::Reverse;
    use std::collections::HashMap;

    fn to_digit(num: i32) -> i32 {
        let mut ret = 0;
        let mut num = num;
        while num != 0 {
            ret += num % 10;
            num /= 10;
        }

        ret
    }

    nums.into_iter()
        .fold(HashMap::new(), |mut acc, n| {
            let digit = to_digit(n);
            acc.entry(digit).or_insert(vec![]).push(n);
            acc
        })
        .into_values()
        .fold(-1, |acc, mut v| {
            if v.len() < 2 {
                acc
            } else {
                v.sort_unstable_by_key(|n| Reverse(*n));
                std::cmp::max(acc, v[0] + v[1])
            }
        })
}
fn main() {
    let nums = vec![18, 43, 36, 13, 7];
    let ret = maximum_sum(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![18, 43, 36, 13, 7];
        let ret = maximum_sum(nums);
        assert_eq!(ret, 54);
    }
    {
        let nums = vec![10, 12, 19, 14];
        let ret = maximum_sum(nums);
        assert_eq!(ret, -1);
    }
}
