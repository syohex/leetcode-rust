fn count_consistent(nums: Vec<i32>) -> i32 {
    let mut ret = 0;
    let len = nums.len();
    for i in 0..len {
        let mut min = i32::MAX;
        let mut and = nums[i];
        for j in i..len {
            min = std::cmp::min(min, nums[j]);
            and &= nums[j];

            if min == and {
                ret += 1;
            }
        }
    }
    ret
}
fn main() {
    let ret = count_consistent(vec![5, 3, 1, 2]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(count_consistent(vec![5, 3, 1, 2]), 6);
    assert_eq!(count_consistent(vec![1, 1, 1]), 6);
    assert_eq!(count_consistent(vec![0]), 1);
}
