fn maximum_sum(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut h: HashMap<i32, Vec<i32>> = HashMap::new();
    for num in nums {
        let mut sum = 0;
        let mut n = num;
        while n > 0 {
            sum += n % 10;
            n /= 10;
        }

        if let Some(v) = h.get_mut(&sum) {
            if num > v[0] {
                v[1] = v[0];
                v[0] = num;
            } else if num > v[1] {
                v[1] = num;
            }
        } else {
            h.insert(sum, vec![num, -1]);
        }
    }

    h.values().fold(-1, |acc, v| {
        if v[1] == -1 {
            acc
        } else {
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
fn test_maximum_sum() {
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
