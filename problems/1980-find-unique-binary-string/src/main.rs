fn find_different_binary_string(nums: Vec<String>) -> String {
    use std::collections::HashSet;

    fn to_num(num: &str) -> i32 {
        num.bytes().fold(0, |acc, b| acc * 2 + (b - b'0') as i32)
    }

    let limit = 2_i32.pow(nums.len() as u32);
    let s: HashSet<i32> = nums.iter().map(|s| to_num(s)).collect();
    let n = (0..limit).find(|&n| !s.contains(&n)).unwrap();
    format!("{:0width$b}", n, width = nums.len())
}

fn main() {
    let nums = vec!["111".to_string(), "011".to_string(), "001".to_string()];
    let ret = find_different_binary_string(nums);
    println!("ret={ret}");
}

#[test]
fn test_find_different_binary_string() {
    {
        let nums = vec!["01".to_string(), "10".to_string()];
        let ret = find_different_binary_string(nums.clone());
        assert!(ret.len() == nums.len());
        assert!(ret.chars().all(|c| c == '0' || c == '1'));
        assert!(nums.iter().find(|&s| *s == ret).is_none());
    }
    {
        let nums = vec!["00".to_string(), "01".to_string()];
        let ret = find_different_binary_string(nums.clone());
        assert!(ret.len() == nums.len());
        assert!(ret.chars().all(|c| c == '0' || c == '1'));
        assert!(nums.iter().find(|&s| *s == ret).is_none());
    }
    {
        let nums = vec!["111".to_string(), "011".to_string(), "001".to_string()];
        let ret = find_different_binary_string(nums.clone());
        assert!(ret.len() == nums.len());
        assert!(ret.chars().all(|c| c == '0' || c == '1'));
        assert!(nums.iter().find(|&s| *s == ret).is_none());
    }
    {
        let nums = vec!["00".to_string(), "01".to_string()];
        let ret = find_different_binary_string(nums.clone());
        assert!(ret.len() == nums.len());
        assert!(ret.chars().all(|c| c == '0' || c == '1'));
        assert!(nums.iter().find(|&s| *s == ret).is_none());
    }
    {
        let nums = vec![
            "0000000111".to_string(),
            "0000001001".to_string(),
            "0000000100".to_string(),
            "0000000001".to_string(),
            "0000000010".to_string(),
            "1111111111".to_string(),
            "0000000101".to_string(),
            "0000000000".to_string(),
            "0000001000".to_string(),
            "0000000110".to_string(),
        ];
        let ret = find_different_binary_string(nums.clone());
        assert!(ret.len() == nums.len());
        assert!(ret.chars().all(|c| c == '0' || c == '1'));
        assert!(nums.iter().find(|&s| *s == ret).is_none());
    }
}
