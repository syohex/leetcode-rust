fn find_different_binary_string(nums: Vec<String>) -> String {
    let v: Vec<Vec<_>> = nums.into_iter().map(|s| s.chars().collect()).collect();
    let mut ret = String::new();
    for (i, cs) in v.into_iter().enumerate() {
        ret.push(if cs[i] == '0' { '1' } else { '0' });
    }

    ret
}

fn main() {
    let nums = vec!["111".to_string(), "011".to_string(), "001".to_string()];
    let ret = find_different_binary_string(nums.clone());
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec!["01".to_string(), "10".to_string()];
        let ret = find_different_binary_string(nums.clone());
        assert!(!nums.contains(&ret));
    }
    {
        let nums = vec!["00".to_string(), "01".to_string()];
        let ret = find_different_binary_string(nums.clone());
        assert!(!nums.contains(&ret));
    }
    {
        let nums = vec!["111".to_string(), "011".to_string(), "001".to_string()];
        let ret = find_different_binary_string(nums.clone());
        assert!(!nums.contains(&ret));
    }
}
