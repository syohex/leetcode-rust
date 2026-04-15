fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
    let len = words.len();
    let start_index = start_index as usize;
    let mut ret = usize::MAX;

    for (i, word) in words.into_iter().enumerate() {
        if word == target {
            let v = if i <= start_index {
                std::cmp::min(start_index - i, i + len - start_index)
            } else {
                std::cmp::min(start_index + len - i, i - start_index)
            };

            ret = std::cmp::min(ret, v);
        }
    }

    if ret == usize::MAX { -1 } else { ret as i32 }
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
    let ret = closest_target(words, target, 1);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let words = vec![
            "hello".to_string(),
            "i".to_string(),
            "am".to_string(),
            "leetcode".to_string(),
            "hello".to_string(),
        ];
        let target = "hello".to_string();
        let ret = closest_target(words, target, 1);
        assert_eq!(ret, 1);
    }
    {
        let words = vec!["a".to_string(), "b".to_string(), "leetcode".to_string()];
        let target = "leetcode".to_string();
        let ret = closest_target(words, target, 0);
        assert_eq!(ret, 1);
    }
    {
        let words = vec![
            "i".to_string(),
            "eat".to_string(),
            "leetcode".to_string(),
            "hello".to_string(),
        ];
        let target = "ate".to_string();
        let ret = closest_target(words, target, 0);
        assert_eq!(ret, -1);
    }
}
