fn can_sort_array(nums: Vec<i32>) -> bool {
    let mut orig = nums.clone();
    let mut nums: Vec<(i32, u32)> = nums.into_iter().map(|n| (n, n.count_ones())).collect();

    orig.sort_unstable();
    nums.sort_by(|(a, ones1), (b, ones2)| {
        if ones1 != ones2 {
            std::cmp::Ordering::Equal
        } else {
            a.cmp(b)
        }
    });

    (0..orig.len()).all(|i| orig[i] == nums[i].0)
}

fn main() {
    let nums = vec![8, 4, 2, 30, 15];
    let ret = can_sort_array(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![8, 4, 2, 30, 15];
        assert!(can_sort_array(nums));
    }
    {
        let nums = vec![1, 2, 3, 4, 5];
        assert!(can_sort_array(nums));
    }
    {
        let nums = vec![3, 16, 8, 4, 2];
        assert!(!can_sort_array(nums));
    }
}
