fn max_absolute_sum(nums: Vec<i32>) -> i32 {
    let mut accs: Vec<i32> = vec![0; nums.len() + 1];
    for (i, &n) in nums.iter().enumerate() {
        accs[i + 1] = accs[i] + n;
    }

    let mut ret = 0;
    for (i, &acc) in accs.iter().enumerate() {
        for j in 0..i {
            ret = std::cmp::max(ret, (acc - accs[j]).abs())
        }
    }

    ret
}

fn main() {
    println!("example1={}", max_absolute_sum(vec![1, -3, 2, 3, -4]));
}

#[test]
fn test_max_absolute_sum() {
    assert_eq!(max_absolute_sum(vec![1, -3, 2, 3, -4]), 5);
    assert_eq!(max_absolute_sum(vec![2, -5, 1, -4, 3, -2]), 8);
}
