fn find_numbers(nums: Vec<i32>) -> i32 {
    fn even_digits(mut n: i32) -> bool {
        let mut digits = 0;
        while n > 0 {
            n /= 10;
            digits += 1;
        }

        digits % 2 == 0
    }

    nums.into_iter().filter(|&n| even_digits(n)).count() as i32
}

fn main() {
    let nums = vec![12, 345, 2, 6, 7896];
    let ret = find_numbers(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![12, 345, 2, 6, 7896];
        let ret = find_numbers(nums);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![555, 901, 482, 1771];
        let ret = find_numbers(nums);
        assert_eq!(ret, 1);
    }
}
