fn max_rotate_function(nums: Vec<i32>) -> i32 {
    let sum: i32 = nums.iter().sum();

    let mut rotate_sum = 0;
    for (i, n) in nums.iter().enumerate() {
        rotate_sum += i as i32 * n;
    }

    let len = nums.len() as i32;
    let mut ret = rotate_sum;
    for n in nums.into_iter().rev() {
        rotate_sum += sum - n - n * (len - 1);
        ret = std::cmp::max(ret, rotate_sum);
    }

    ret
}

fn main() {
    let ret = max_rotate_function(vec![4, 3, 2, 6]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(max_rotate_function(vec![4, 3, 2, 6]), 26);
    assert_eq!(max_rotate_function(vec![100]), 0);
}
