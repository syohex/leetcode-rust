fn difference_of_sum(nums: Vec<i32>) -> i32 {
    let sum: i32 = nums.iter().sum();
    let digit_sum = nums.iter().fold(0, |acc, n| {
        let mut n = *n;
        let mut sum = 0;
        while n > 0 {
            sum += n % 10;
            n /= 10;
        }

        acc + sum
    });

    (sum - digit_sum).abs()
}

fn main() {
    let nums = vec![1, 15, 6, 3];
    let ret = difference_of_sum(nums);
    println!("ret={ret}");
}

#[test]
fn test_differnece_of_sum() {
    {
        let nums = vec![1, 15, 6, 3];
        let ret = difference_of_sum(nums);
        assert_eq!(ret, 9);
    }
    {
        let nums = vec![1, 2, 3, 4];
        let ret = difference_of_sum(nums);
        assert_eq!(ret, 0);
    }
}
