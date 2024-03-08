fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut h: HashMap<i32, i32> = HashMap::new();
    let mut max = 0;
    for num in nums {
        *h.entry(num).or_insert(0) += 1;
        max = std::cmp::max(max, *h.get(&num).unwrap());
    }

    let mut ret = 0;
    for v in h.values() {
        if *v == max {
            ret += v;
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
