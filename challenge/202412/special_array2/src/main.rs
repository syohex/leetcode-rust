fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let mut same_pairs = vec![];

    for i in 1..nums.len() {
        if nums[i-1] % 2 == nums[i] %2 {
            same_pairs.push(i as i32);
        }
    }

    queries
    .into_iter()
    .map(|v| {
        let (start, end) = (v[0], v[1]);
        for &i in &same_pairs {
            if end < i {
                continue;
            }
            if start > end {
                break;
            }

            if start < i && i <= end {
                return false;
            }
        }
        true
    }).collect()
}

fn main() {
    let nums = vec![4, 3, 1, 6];
    let queries = vec![vec![0, 2], vec![2, 3]];
    let ret = is_array_special(nums, queries);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let nums = vec![3, 4, 1, 2, 6];
        let queries = vec![vec![0, 4]];
        let expected = vec![false];
        let ret = is_array_special(nums, queries);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![4, 3, 1, 6];
        let queries = vec![vec![0, 2], vec![2, 3]];
        let expected = vec![false, true];
        let ret = is_array_special(nums, queries);
        assert_eq!(ret, expected);
    }
}
