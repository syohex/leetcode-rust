fn max_digit_range(nums: Vec<i32>) -> i32 {
    fn f(num: i32) -> i32 {
        if num == 0 {
            0
        } else {
            let mut num = num;
            let mut min = i32::MAX;
            let mut max = i32::MIN;
            while num > 0 {
                let m = num % 10;
                min = std::cmp::min(min, m);
                max = std::cmp::max(max, m);
                num /= 10;
            }
            max - min
        }
    }

    let mut max_range = 0;
    let mut sum = [0; 10];
    for n in nums {
        let d = f(n);
        max_range = std::cmp::max(max_range, d);
        sum[d as usize] += n;
    }

    sum[max_range as usize]
}

fn main() {
    let ret = max_digit_range(vec![5724, 111, 350]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(max_digit_range(vec![5724, 111, 350]), 6074);
    assert_eq!(max_digit_range(vec![90, 900]), 990);
}
