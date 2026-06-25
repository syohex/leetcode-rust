fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i32 {
    let len = nums.len();
    let mut ret = 0;
    for i in 0..len {
        let mut count = 0;
        for j in i..len {
            count += if nums[j] == target { 1 } else { -1 };

            if count > 0 {
                ret += 1;
            }
        }
    }

    ret
}

fn main() {
    let ret = count_majority_subarrays(vec![1, 2, 2, 3], 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(count_majority_subarrays(vec![1, 2, 2, 3], 2), 5);
    assert_eq!(count_majority_subarrays(vec![1, 1, 1, 1], 1), 10);
    assert_eq!(count_majority_subarrays(vec![1, 2, 3], 4), 0);
}
