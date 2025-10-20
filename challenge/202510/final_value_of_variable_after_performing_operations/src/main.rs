fn final_value_after_operations(operations: Vec<String>) -> i32 {
    operations.into_iter().fold(0, |acc, s| {
        if let Some(c) = s.chars().nth(1)
            && c == '+'
        {
            acc + 1
        } else {
            acc - 1
        }
    })
}

fn main() {
    let operations = vec!["--X".to_string(), "X++".to_string(), "X++".to_string()];
    let ret = final_value_after_operations(operations);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let operations = vec!["--X".to_string(), "X++".to_string(), "X++".to_string()];
        let ret = final_value_after_operations(operations);
        assert_eq!(ret, 1);
    }
    {
        let operations = vec!["++X".to_string(), "++X".to_string(), "X++".to_string()];
        let ret = final_value_after_operations(operations);
        assert_eq!(ret, 3);
    }
    {
        let operations = vec![
            "X++".to_string(),
            "++X".to_string(),
            "--X".to_string(),
            "X--".to_string(),
        ];
        let ret = final_value_after_operations(operations);
        assert_eq!(ret, 0);
    }
}
