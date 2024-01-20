fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut h = HashMap::new();
    let mut max = 1;
    for num in nums {
        if let Some(v) = h.get_mut(&num) {
            *v += 1;
            max = std::cmp::max(max, *v);
        } else {
            h.insert(num, 1);
        }
    }

    let mut ret = 0;
    for v in h.values() {
        if *v == max {
            ret += max;
        }
    }

    ret
}
fn main() {
    let nums = vec![1, 2, 2, 3, 1, 4];
    let ret = max_frequency_elements(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 2, 2, 3, 1, 4];
        let ret = max_frequency_elements(nums);
        assert_eq!(ret, 4);
    }
    {
        let nums = vec![1, 2, 3, 4, 5];
        let ret = max_frequency_elements(nums);
        assert_eq!(ret, 5);
    }
}
