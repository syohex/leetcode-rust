fn find_different_binary_string(nums: Vec<String>) -> String {
    let nums: Vec<Vec<char>> = nums.into_iter().map(|s| s.chars().collect()).collect();

    let mut ret = String::new();
    for i in 0..nums.len() {
        if nums[i][i] == '0' {
            ret.push('1');
        } else {
            ret.push('0');
        }
    }
    ret
}
fn main() {
    let nums = vec!["111".to_string(), "011".to_string(), "001".to_string()];
    let ret = find_different_binary_string(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec!["01".to_string(), "10".to_string()];
        let ret = find_different_binary_string(nums.clone());
        assert!(!nums.iter().any(|s| *s == ret));
    }
    {
        let nums = vec!["00".to_string(), "01".to_string()];
        let ret = find_different_binary_string(nums.clone());
        assert!(!nums.iter().any(|s| *s == ret));
    }
    {
        let nums = vec!["111".to_string(), "011".to_string(), "001".to_string()];
        let ret = find_different_binary_string(nums.clone());
        assert!(!nums.iter().any(|s| *s == ret));
    }
    {
        let nums = vec![
            "1001".to_string(),
            "1000".to_string(),
            "0110".to_string(),
            "1111".to_string(),
        ];
        let ret = find_different_binary_string(nums.clone());
        assert!(!nums.iter().any(|s| *s == ret));
    }
    {
        let nums = vec!["000".to_string(), "001".to_string(), "110".to_string()];
        let ret = find_different_binary_string(nums.clone());
        assert!(!nums.iter().any(|s| *s == ret));
    }
}
