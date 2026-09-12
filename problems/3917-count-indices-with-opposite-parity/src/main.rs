fn count_opposite_parity(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut ret = vec![0; len];

    for i in 0..len {
        let mut score = 0;
        let is_even = nums[i] % 2 == 0;
        for j in (i + 1)..len {
            let is_even2 = nums[j] % 2 == 0;
            if is_even != is_even2 {
                score += 1;
            }
        }

        ret[i] = score;
    }

    ret
}

fn main() {
    let ret = count_opposite_parity(vec![1, 2, 3, 4]);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    assert_eq!(count_opposite_parity(vec![1, 2, 3, 4]), vec![2, 1, 1, 0]);
    assert_eq!(count_opposite_parity(vec![1]), vec![0]);
}
