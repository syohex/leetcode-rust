fn cal_points(ops: Vec<String>) -> i32 {
    let mut stack = vec![];

    for op in ops {
        let len = stack.len();
        match op.as_str() {
            "+" => {
                let num1 = stack[len - 2];
                let num2 = stack[len - 1];
                stack.push(num1 + num2);
            }
            "D" => {
                let num = stack[len - 1];
                stack.push(num * 2);
            }
            "C" => {
                stack.pop();
            }
            s => {
                let num = s.parse::<i32>().unwrap();
                stack.push(num);
            }
        }
    }

    stack.iter().sum()
}

fn main() {
    let ops = vec![
        "5".to_string(),
        "2".to_string(),
        "C".to_string(),
        "D".to_string(),
        "+".to_string(),
    ];
    let ret = cal_points(ops);
    println!("ret={ret}");
}

#[test]
fn test_cal_points() {
    {
        let ops = vec![
            "5".to_string(),
            "2".to_string(),
            "C".to_string(),
            "D".to_string(),
            "+".to_string(),
        ];
        let ret = cal_points(ops);
        assert_eq!(ret, 30);
    }
    {
        let ops = vec![
            "5".to_string(),
            "-2".to_string(),
            "4".to_string(),
            "C".to_string(),
            "D".to_string(),
            "9".to_string(),
            "+".to_string(),
            "+".to_string(),
        ];
        let ret = cal_points(ops);
        assert_eq!(ret, 27);
    }
}
