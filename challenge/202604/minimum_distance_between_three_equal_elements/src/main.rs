fn minimum_distance(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut ret = usize::MAX;
    for i in 0..len {
        for j in (i + 1)..len {
            if nums[i] != nums[j] {
                continue;
            }
            for k in (j + 1)..len {
                if nums[k] != nums[j] {
                    continue;
                }

                ret = std::cmp::min(ret, j - i + k - j + k - i);
            }
        }
    }

    if ret == usize::MAX { -1 } else { ret as i32 }
}
fn main() {
    let ret = minimum_distance(vec![1, 1, 2, 3, 2, 1, 2]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(minimum_distance(vec![1, 2, 1, 1, 3]), 6);
    assert_eq!(minimum_distance(vec![1, 1, 2, 3, 2, 1, 2]), 8);
    assert_eq!(minimum_distance(vec![1]), -1);
}
