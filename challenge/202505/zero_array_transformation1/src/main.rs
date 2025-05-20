fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
    let mut prefix = vec![0; nums.len() + 1];
    for q in &queries {
        prefix[q[0] as usize] += 1;
        prefix[q[1] as usize + 1] -= 1;
    }

    let mut v = 0;
    let mut values = vec![];
    for p in prefix {
        v += p;
        values.push(v);
    }

    for i in 0..nums.len() {
        if nums[i] > values[i] {
            return false;
        }
    }

    true
}

fn main() {
    let nums = vec![1, 0, 1];
    let queries = vec![vec![0, 2]];
    let ret = is_zero_array(nums, queries);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 0, 1];
        let queries = vec![vec![0, 2]];
        let ret = is_zero_array(nums, queries);
        assert!(ret);
    }
    {
        let nums = vec![4, 3, 2, 1];
        let queries = vec![vec![1, 3], vec![0, 2]];
        let ret = is_zero_array(nums, queries);
        assert!(!ret);
    }
}
