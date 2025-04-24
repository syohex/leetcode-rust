fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
    use std::collections::{HashMap, HashSet};

    let len = nums.len();
    let uniques = nums.iter().cloned().collect::<HashSet<i32>>().len();
    let mut h = HashMap::new();

    let mut ret = 0;
    let mut right = 0;
    for i in 0..len {
        if i > 0 {
            let mut should_remove = false;
            if let Some(v) = h.get_mut(&nums[i - 1]) {
                *v -= 1;
                if *v == 0 {
                    should_remove = true;
                }
            }
            if should_remove {
                h.remove(&nums[i - 1]);
            }
        }

        while right < len && h.len() < uniques {
            *h.entry(nums[right]).or_insert(0) += 1;
            right += 1;
        }

        if h.len() == uniques {
            ret += len - right + 1;
        }
    }

    ret as i32
}

fn main() {
    let nums = vec![1, 3, 1, 2, 2];
    let ret = count_complete_subarrays(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 3, 1, 2, 2];
        let ret = count_complete_subarrays(nums);
        assert_eq!(ret, 4);
    }
    {
        let nums = vec![5, 5, 5, 5];
        let ret = count_complete_subarrays(nums);
        assert_eq!(ret, 10);
    }
}
