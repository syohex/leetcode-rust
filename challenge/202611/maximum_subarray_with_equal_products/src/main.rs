fn max_length(nums: Vec<i32>) -> i32 {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let m = a % b;
            a = b;
            b = m;
        }

        a
    }

    let mut left = 0;
    let mut right = 0;

    let mut ret = 2;
    let mut prod = 1;
    while right < nums.len() {
        while gcd(prod, nums[right]) != 1 {
            prod /= nums[left];
            left += 1;
        }

        prod *= nums[right];
        ret = std::cmp::max(ret, right - left + 1);
        right += 1;
    }

    ret as i32
}

fn main() {
    let ret = max_length(vec![1, 2, 1, 2, 1, 1, 1]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(max_length(vec![2, 6]), 2);
    assert_eq!(max_length(vec![1, 2, 1, 2, 1, 1, 1]), 5);
    assert_eq!(max_length(vec![2, 3, 4, 5, 6]), 3);
    assert_eq!(max_length(vec![1, 2, 3, 1, 4, 5, 1]), 5);
}
