fn min_element(nums: Vec<i32>) -> i32 {
    nums.into_iter()
        .map(|n| {
            let mut n = n;
            let mut sum = 0;
            while n > 0 {
                sum += n % 10;
                n /= 10;
            }

            sum
        })
        .min()
        .unwrap()
}

fn main() {
    let nums = vec![10, 12, 13, 14];
    let ret = min_element(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![10, 12, 13, 14];
        let ret = min_element(nums);
        assert_eq!(ret, 1);
    }
    {
        let nums = vec![999, 19, 199];
        let ret = min_element(nums);
        assert_eq!(ret, 10);
    }
    {
        let nums = vec![1, 2, 3, 4];
        let ret = min_element(nums);
        assert_eq!(ret, 1);
    }
}
