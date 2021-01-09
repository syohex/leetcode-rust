fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut count = 0i32;

    for i in 0..(nums.len() - 1) {
        for j in (i + 1)..nums.len() {
            if nums[i] == nums[j] {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    println!(
        "num_identical_pairs[1, 2, 3, 1, 1, 3] == {}",
        num_identical_pairs(vec![1, 2, 3, 1, 1, 3])
    );
}

#[test]
fn test_num_identical_pairs() {
    assert_eq!(num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
    assert_eq!(num_identical_pairs(vec![1, 1, 1, 1]), 6);
    assert_eq!(num_identical_pairs(vec![1, 2, 3]), 0);
}
