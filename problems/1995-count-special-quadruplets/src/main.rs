fn count_quadruplets(nums: Vec<i32>) -> i32 {
    let len = nums.len();

    let mut ret = 0;
    for a in 0..len - 3 {
        for b in (a + 1)..len - 2 {
            for c in (b + 1)..len - 1 {
                for d in (c + 1)..len {
                    if nums[a] + nums[b] + nums[c] == nums[d] {
                        ret += 1;
                    }
                }
            }
        }
    }

    ret
}

fn main() {
    let nums = vec![1, 2, 3, 6];
    println!("ret={}", count_quadruplets(nums));
}

#[test]
fn test_count_quadruplets() {
    assert_eq!(count_quadruplets(vec![1, 2, 3, 6]), 1);
    assert_eq!(count_quadruplets(vec![3, 3, 6, 4, 5]), 0);
    assert_eq!(count_quadruplets(vec![1, 1, 1, 3, 5]), 4);
}
