fn final_value_after_operations(operations: Vec<String>) -> i32 {
    operations.iter().fold(0, |acc, op| {
        if op == "X++" || op == "++X" {
            acc + 1
        } else {
            acc - 1
        }
    })
}

fn main() {
    let operations = vec!["--X".to_string(), "X++".to_string(), "X++".to_string()];
    let ret = final_value_after_operations(operations);
    println!("ret={}", ret);
}

#[test]
fn test_final_value_after_operations() {
    {
        let operations = vec!["--X".to_string(), "X++".to_string(), "X++".to_string()];
        assert_eq!(final_value_after_operations(operations), 1);
    }
    {
        let operations = vec!["++X".to_string(), "++X".to_string(), "X++".to_string()];
        assert_eq!(final_value_after_operations(operations), 3);
    }
    {
        let operations = vec![
            "X++".to_string(),
            "++X".to_string(),
            "--X".to_string(),
            "X--".to_string(),
        ];
        assert_eq!(final_value_after_operations(operations), 0);
    }
}
