fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    use std::collections::HashSet;

    let n = nums.len() as i32;
    let mut s: HashSet<i32> = (1..=n).into_iter().collect();
    let mut duplicated = 0;
    for num in nums {
        if s.contains(&num) {
            s.remove(&num);
        } else {
            duplicated = num;
        }
    }

    vec![duplicated, s.into_iter().next().unwrap()]
}

fn main() {
    let nums = vec![1, 2, 2, 4];
    let ret = find_error_nums(nums);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 2, 2, 4];
        let expected = vec![2, 3];
        let ret = find_error_nums(nums);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![1, 1];
        let expected = vec![1, 2];
        let ret = find_error_nums(nums);
        assert_eq!(ret, expected);
    }
}
