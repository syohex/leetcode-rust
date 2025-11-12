fn min_operations(nums: Vec<i32>) -> i32 {
    fn gcd(a: i32, b: i32) -> i32 {
        let (mut a, mut b) = if a >= b { (a, b) } else { (b, a) };
        while b != 0 {
            let m = a % b;
            a = b;
            b = m;
        }

        a
    }

    let len = nums.len();
    let mut d = nums[0];
    let mut ones = 0;
    for &n in nums.iter() {
        d = gcd(d, n);
        if n == 1 {
            ones += 1;
        }
    }

    if ones != 0 {
        return len as i32 - ones;
    }

    if d != 1 {
        return -1;
    }

    let mut min_len = len;
    for i in 0..len {
        let mut d = nums[i];
        for j in i..len {
            d = gcd(d, nums[j]);
            if d == 1 {
                min_len = std::cmp::min(min_len, j - i + 1);
                break;
            }
        }
    }

    (min_len as i32 - 1) + (len as i32 - 1)
}

fn main() {
    let nums = vec![2, 6, 3, 4];
    let ret = min_operations(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![2, 6, 3, 4];
        let ret = min_operations(nums);
        assert_eq!(ret, 4);
    }
    {
        let nums = vec![2, 10, 6, 14];
        let ret = min_operations(nums);
        assert_eq!(ret, -1);
    }
}
