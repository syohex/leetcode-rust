fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
    use std::collections::HashSet;

    let h: HashSet<i32> = nums.into_iter().collect();
    let mut ret = original;
    loop {
        if h.contains(&ret) {
            ret *= 2;
        } else {
            return ret;
        }
    }
}

fn main() {
    let nums = vec![5, 3, 6, 1, 12];
    let ret = find_final_value(nums, 3);
    println!("ret={ret}");
}

#[test]
fn test_find_final_value() {
    {
        let nums = vec![5, 3, 6, 1, 12];
        assert_eq!(find_final_value(nums, 3), 24);
    }
    {
        let nums = vec![8, 19, 4, 2, 15, 3];
        assert_eq!(find_final_value(nums, 2), 16);
    }
    {
        let nums = vec![2, 7, 9];
        assert_eq!(find_final_value(nums, 4), 4);
    }
}
