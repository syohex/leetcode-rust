fn count_nice_pairs(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    fn rev(n: i32) -> i32 {
        let mut n = n;
        let mut ret = 0;
        while n > 0 {
            ret = ret * 10 + n % 10;
            n /= 10;
        }

        ret
    }

    let diffs: Vec<_> = nums.into_iter().map(|n| n - rev(n)).collect();
    let mut h = HashMap::new();
    let mut ret = 0;

    let modulo = 1_000_000_007;
    for diff in diffs {
        ret = (ret + h.get(&diff).unwrap_or(&0)) % modulo;
        *h.entry(diff).or_insert(0) += 1;
    }

    ret
}

fn main() {
    let nums = vec![13, 10, 35, 24, 76];
    let ret = count_nice_pairs(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![42, 11, 1, 97];
        let ret = count_nice_pairs(nums);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![13, 10, 35, 24, 76];
        let ret = count_nice_pairs(nums);
        assert_eq!(ret, 4);
    }
}
