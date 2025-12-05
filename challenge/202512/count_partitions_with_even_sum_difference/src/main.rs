fn count_partition(nums: Vec<i32>) -> i32 {
    let mut ret = 0;
    let mut left = 0;
    let mut right: i32 = nums.iter().sum();

    let len = nums.len();
    for n in nums.into_iter().take(len - 1) {
        left += n;
        right -= n;

        if (left - right) % 2 == 0 {
            ret += 1;
        }
    }

    ret
}

fn main() {
    let ret = count_partition(vec![10, 10, 3, 7, 6]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(count_partition(vec![10, 10, 3, 7, 6]), 4);
    assert_eq!(count_partition(vec![1, 2, 2]), 0);
    assert_eq!(count_partition(vec![2, 4, 6, 8]), 3);
}
