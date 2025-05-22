fn smallest_index(nums: Vec<i32>) -> i32 {
    match nums.into_iter().enumerate().position(|(i, mut n)| {
        let mut sum = 0;
        while n > 0 {
            sum += n % 10;
            n /= 10;
        }
        sum as usize == i
    }) {
        Some(v) => v as i32,
        None => -1,
    }
}

fn main() {
    let nums = vec![1, 3, 2];
    let ret = smallest_index(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 3, 2];
        let ret = smallest_index(nums);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![1, 10, 11];
        let ret = smallest_index(nums);
        assert_eq!(ret, 1);
    }
}
