fn is_array_special(nums: Vec<i32>) -> bool {
    if nums.len() == 1 {
        true
    } else {
        nums.windows(2).all(|v| v[0] % 2 != v[1] % 2)
    }
}

fn main() {
    let nums = vec![4, 3, 1, 6];
    let ret = is_array_special(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1];
        assert!(is_array_special(nums));
    }
    {
        let nums = vec![2, 1, 4];
        assert!(is_array_special(nums));
    }
    {
        let nums = vec![4, 3, 1, 6];
        assert!(!is_array_special(nums));
    }
}
