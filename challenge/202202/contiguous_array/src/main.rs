fn find_max_length(nums: Vec<i32>) -> i32 {
    use std::cmp::max;
    use std::collections::HashMap;

    let mut h: HashMap<i32, i32> = HashMap::new();
    h.insert(0, -1);

    let mut sum = 0;
    let mut ret = 0;
    for (i, n) in nums.iter().enumerate() {
        let i = i as i32;
        sum += if *n == 1 { 1 } else { -1 };
        if let Some(j) = h.get(&sum) {
            ret = max(ret, i - j);
        } else {
            h.insert(sum, i);
        }
    }

    ret
}

fn main() {
    let nums = vec![0, 1, 0];
    let ret = find_max_length(nums);
    println!("ret={ret}");
}

#[test]
fn test_find_max_length() {
    {
        let nums = vec![0, 1];
        assert_eq!(find_max_length(nums), 2);
    }
    {
        let nums = vec![0, 1, 0];
        assert_eq!(find_max_length(nums), 2);
    }
}
