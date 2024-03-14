fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
    use std::collections::HashMap;

    let mut h = HashMap::new();
    let mut sum = 0;
    let mut ret = 0;

    for num in nums {
        sum += num;
        if sum == goal {
            ret += 1;
        }

        let diff = sum - goal;
        if let Some(&v) = h.get(&diff) {
            ret += v;
        }

        *h.entry(sum).or_insert(0) += 1;
    }

    ret
}

fn main() {
    let nums = vec![0, 0, 0, 0, 0];
    let ret = num_subarrays_with_sum(nums, 0);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 0, 1, 0, 1];
        let ret = num_subarrays_with_sum(nums, 2);
        assert_eq!(ret, 4);
    }
    {
        let nums = vec![0, 0, 0, 0, 0];
        let ret = num_subarrays_with_sum(nums, 0);
        assert_eq!(ret, 15);
    }
}
