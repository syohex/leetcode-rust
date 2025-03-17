fn divide_array(nums: Vec<i32>) -> bool {
    nums.into_iter()
        .fold(std::collections::HashMap::new(), |mut acc, n| {
            *acc.entry(n).or_insert(0) += 1;
            acc
        })
        .into_values()
        .all(|n| n % 2 == 0)
}

fn main() {
    let nums = vec![3, 2, 3, 2, 2, 2];
    let ret = divide_array(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![3, 2, 3, 2, 2, 2];
        assert!(divide_array(nums));
    }
    {
        let nums = vec![1, 2, 3, 4];
        assert!(!divide_array(nums));
    }
}
