pub fn closet_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
    let start_index = start_index as usize;
    let poss: Vec<i32> = words
        .iter()
        .enumerate()
        .filter(|(_, s)| **s == target)
        .map(|(i, _)| {
            let a = (i as i32 - start_index as i32).abs();
            let b = if i <= start_index {
                (i + words.len() - start_index) as i32
            } else {
                (start_index + words.len() - i) as i32
            };
            std::cmp::min(a, b)
        })
        .collect();

    if poss.is_empty() {
        -1
    } else {
        poss.into_iter().min().unwrap()
    }
}

fn main() {
    let words = vec![
        "hello".to_string(),
        "i".to_string(),
        "am".to_string(),
        "leetcode".to_string(),
        "hello".to_string(),
    ];
    let target = "hello".to_string();
    let start_index = 1;
    let ret = closet_target(words, target, start_index);
    println!("ret={ret}");
}

#[test]
fn test_closet_target() {
    {
        let words = vec![
            "hello".to_string(),
            "i".to_string(),
            "am".to_string(),
            "leetcode".to_string(),
            "hello".to_string(),
        ];
        let target = "hello".to_string();
        let start_index = 1;
        let ret = closet_target(words, target, start_index);
        assert_eq!(ret, 1);
    }
    {
        let words = vec!["a".to_string(), "b".to_string(), "leetcode".to_string()];
        let target = "leetcode".to_string();
        let start_index = 0;
        let ret = closet_target(words, target, start_index);
        assert_eq!(ret, 1);
    }
    {
        let words = vec![
            "odjrjznxpn".to_string(),
            "cyulttuabe".to_string(),
            "zqxkdoeszk".to_string(),
            "yeewpgriok".to_string(),
            "odjrjznxpn".to_string(),
            "btqpvxpjzv".to_string(),
            "ukyudladhk".to_string(),
            "ukyudladhk".to_string(),
            "odjrjznxpn".to_string(),
            "yeewpgriok".to_string(),
        ];
        let target = "odjrjznxpn".to_string();
        let start_index = 5;
        let ret = closet_target(words, target, start_index);
        assert_eq!(ret, 1);
    }
}
