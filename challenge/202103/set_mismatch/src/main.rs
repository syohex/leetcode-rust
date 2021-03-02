fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let mut v = vec![0; nums.len()];
    for &n in &nums {
        let index = (n - 1) as usize;
        v[index] = v[index] + 1;
    }

    let mut repeated = 0;
    let mut missed = 0;
    for i in 0..nums.len() {
        if v[i] == 2 {
            repeated = (i + 1) as i32;
        } else if v[i] == 0 {
            missed = (i + 1) as i32;
        }
    }

    vec![repeated, missed]
}

fn main() {
    let ret = find_error_nums(vec![1, 2, 2, 4]);
    println!("ret={:?}", ret);
}

#[test]
fn test_find_error_nums() {
    assert_eq!(find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
    assert_eq!(find_error_nums(vec![1, 1]), vec![1, 2]);
}
