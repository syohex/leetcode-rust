fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
    use std::collections::HashSet;

    let banned : HashSet<_> = banned.into_iter().collect();

    let mut left = 0;
    let mut sum = 0;
    let mut ret = 0;
    let mut nums = 0;
    for i in 1..=n {
        if banned.contains(&i) {
            continue;
        }

        sum += i;
        nums += 1;
        while left <= i && sum >= max_sum {
            if !banned.contains(&left) {
                sum -= left;
                nums -= 1;
            }

            left += 1;
        }

        ret = std::cmp::max(ret, nums);
    }

    ret
}

fn main() {
    let banned = vec![1, 6, 5];
    let n = 5;
    let max_sum = 6;
    let ret = max_count(banned, n, max_sum);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let banned = vec![1, 6, 5];
        let n = 5;
        let max_sum = 6;
        let ret = max_count(banned, n, max_sum);
        assert_eq!(ret, 2);
    }
    {
        let banned = vec![1, 2, 3, 4, 5, 6, 7];
        let n = 8;
        let max_sum = 1;
        let ret = max_count(banned, n, max_sum);
        assert_eq!(ret, 0);
    }
    {
        let banned = vec![11];
        let n = 7;
        let max_sum = 50;
        let ret = max_count(banned, n, max_sum);
        assert_eq!(ret, 7);
    }
}
