fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
    let total_minutes = batteries.iter().fold(0i64, |acc, &n| acc + n as i64);

    let n = n as i64;
    let mut left = 0;
    let mut right = total_minutes / n;

    while left < right {
        let mid = right - (right - left) / 2;

        let mut sum = 0i64;
        for &b in &batteries {
            sum += std::cmp::min(b as i64, mid);
        }

        if sum >= mid * n {
            left = mid;
        } else {
            right = mid - 1;
        }
    }

    left
}

fn main() {
    let batteries = vec![3, 3, 3];
    let ret = max_run_time(2, batteries);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let batteries = vec![3, 3, 3];
        let ret = max_run_time(2, batteries);
        assert_eq!(ret, 4);
    }
    {
        let batteries = vec![1, 1, 1, 1];
        let ret = max_run_time(2, batteries);
        assert_eq!(ret, 2);
    }
}
