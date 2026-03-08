fn find_different_binary_string(nums: Vec<String>) -> String {
    let v: Vec<Vec<_>> = nums.into_iter().map(|s| s.chars().collect()).collect();
    let mut ret = String::new();
    for (i, cs) in v.into_iter().enumerate() {
        ret.push(if cs[i] == '1' { '0' } else { '1' });
    }

    ret
}

fn main() {
    let ret = find_different_binary_string(vec!["01".to_string(), "10".to_string()]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(
        find_different_binary_string(vec!["01".to_string(), "10".to_string()]),
        "11"
    );
    assert_eq!(
        find_different_binary_string(vec!["00".to_string(), "01".to_string()]),
        "10"
    );
    assert_eq!(
        find_different_binary_string(vec![
            "111".to_string(),
            "011".to_string(),
            "001".to_string()
        ]),
        "000"
    );
}
