fn first_unique_freq(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut freq = HashMap::new();
    let mut pos = HashMap::new();
    for (i, n) in nums.into_iter().enumerate() {
        *freq.entry(n).or_insert(0) += 1;
        pos.entry(n).or_insert(i);
    }

    let mut count = HashMap::new();
    for (k, v) in freq {
        count.entry(v).or_insert(vec![]).push(k);
    }

    let mut min_index = usize::MAX;
    let mut ret = -1;
    for v in count.values() {
        if v.len() == 1
            && let Some(p) = pos.get(&v[0])
            && *p < min_index
        {
            ret = v[0];
            min_index = *p;
        }
    }

    ret
}

fn main() {
    let nums = vec![20, 20, 10, 30, 30, 20];
    let ret = first_unique_freq(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![20, 10, 30, 30];
        let ret = first_unique_freq(nums);
        assert_eq!(ret, 30);
    }
    {
        let nums = vec![20, 20, 10, 30, 30, 20];
        let ret = first_unique_freq(nums);
        assert_eq!(ret, 20);
    }
    {
        let nums = vec![10,10,20,20];
        let ret = first_unique_freq(nums);
        assert_eq!(ret, -1);
    }
}
