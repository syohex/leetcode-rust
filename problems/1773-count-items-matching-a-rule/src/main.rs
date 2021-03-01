fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
    items
        .iter()
        .filter(|&v| -> bool {
            match rule_key.as_str() {
                "type" => v[0] == rule_value,
                "color" => v[1] == rule_value,
                "name" => v[2] == rule_value,
                _ => panic!("never reach here"),
            }
        })
        .count() as i32
}

fn main() {
    let items = vec![
        vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
        vec![
            "computer".to_string(),
            "silver".to_string(),
            "lenovo".to_string(),
        ],
        vec![
            "phone".to_string(),
            "gold".to_string(),
            "iphone".to_string(),
        ],
    ];
    let ret = count_matches(items, "color".to_string(), "silver".to_string());
    println!("ret={}", ret);
}

#[test]
fn test_count_matches() {
    {
        let items = vec![
            vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
            vec![
                "computer".to_string(),
                "silver".to_string(),
                "lenovo".to_string(),
            ],
            vec![
                "phone".to_string(),
                "gold".to_string(),
                "iphone".to_string(),
            ],
        ];
        assert_eq!(
            count_matches(items, "color".to_string(), "silver".to_string()),
            1
        );
    }
    {
        let items = vec![
            vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
            vec![
                "computer".to_string(),
                "silver".to_string(),
                "phone".to_string(),
            ],
            vec![
                "phone".to_string(),
                "gold".to_string(),
                "iphone".to_string(),
            ],
        ];
        assert_eq!(
            count_matches(items, "type".to_string(), "phone".to_string()),
            2
        );
    }
}
