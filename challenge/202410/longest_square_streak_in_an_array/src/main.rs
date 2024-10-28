fn longest_square_streak(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    let s: HashSet<i64> = nums.iter().map(|&n| n as i64).collect();

    let max_len = nums.into_iter().fold(0, |acc, n| {
        let mut len = 1;
        let mut key = n as i64 * n as i64;
        loop {
            if !s.contains(&key) {
                break;
            }

            key = key * key;
            len += 1;
        }

        std::cmp::max(acc, len)
    });

    if max_len == 1 {
        -1
    } else {
        max_len
    }
}

fn main() {
    let nums = vec![4, 3, 6, 16, 8, 2];
    let ret = longest_square_streak(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![4, 3, 6, 16, 8, 2];
        let ret = longest_square_streak(nums);
        assert_eq!(ret, 3);
    }
    {
        let nums = vec![2, 3, 5, 6, 7];
        let ret = longest_square_streak(nums);
        assert_eq!(ret, -1);
    }
}
