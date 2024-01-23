fn max_length(arr: Vec<String>) -> i32 {
    use std::collections::HashSet;

    let mut candidates = vec!["".to_string()];
    let mut ret = 0;

    for s in &arr {
        let len = candidates.len();
        for i in 0..len {
            let new_str = format!("{}{}", s, candidates[i]);
            let hs: HashSet<char> = new_str.chars().collect();
            if new_str.len() != hs.len() {
                continue;
            }

            ret = std::cmp::max(ret, new_str.len());
            candidates.push(new_str);
        }
    }

    ret as i32
}

fn main() {
    let arr = vec![
        "cha".to_string(),
        "r".to_string(),
        "act".to_string(),
        "ers".to_string(),
    ];
    let ret = max_length(arr);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let arr = vec!["un".to_string(), "iq".to_string(), "ue".to_string()];
        let ret = max_length(arr);
        assert_eq!(ret, 4);
    }
    {
        let arr = vec![
            "cha".to_string(),
            "r".to_string(),
            "act".to_string(),
            "ers".to_string(),
        ];
        let ret = max_length(arr);
        assert_eq!(ret, 6);
    }
    {
        let arr = vec!["abcdefghijklmnopqrstuvwxyz".to_string()];
        let ret = max_length(arr);
        assert_eq!(ret, 26);
    }
}
