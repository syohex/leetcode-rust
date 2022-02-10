fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashMap;

    let mut accs = vec![0; nums.len()];
    accs[0] = nums[0];

    for (i, &n) in nums.iter().enumerate().skip(1) {
        accs[i] = n + accs[i - 1];
    }

    let mut ret = 0;
    let mut m: HashMap<i32, i32> = HashMap::new();
    m.insert(0, 1);

    for &acc in &accs {
        let diff = acc - k;
        if let Some(v) = m.get(&diff) {
            ret += v;
        }

        *m.entry(acc).or_insert(0) += 1;
    }

    ret
}

fn main() {
    let nums = vec![1, 2, 3];
    let ret = subarray_sum(nums, 2);
    println!("ret={ret}");
}

#[test]
fn test_subarray_sum() {
    {
        let nums = vec![1, 1, 1];
        assert_eq!(subarray_sum(nums, 2), 2);
    }
    {
        let nums = vec![1, 2, 3];
        assert_eq!(subarray_sum(nums, 3), 2);
    }
}
