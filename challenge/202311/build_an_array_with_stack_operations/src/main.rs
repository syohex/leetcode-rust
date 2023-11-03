fn build_array(target: Vec<i32>, _n: i32) -> Vec<String> {
    target
        .into_iter()
        .fold((vec![], 0), |(mut acc, mut i), num| {
            let diff = (num - 1) as usize - i;
            if diff > 0 {
                (0..diff).into_iter().for_each(|_| {
                    acc.push("Push".to_string());
                    acc.push("Pop".to_string());
                    i += 1;
                });
            }
            acc.push("Push".to_string());
            (acc, i + 1)
        }).0
}

fn main() {
    let target = vec![1, 3];
    let n = 3;
    let ret = build_array(target, n);
    println!("ret={ret:?}");
}

#[test]
fn test_build_array() {
    {
        let target = vec![1, 3];
        let n = 3;
        let expected = vec![
            "Push".to_string(),
            "Push".to_string(),
            "Pop".to_string(),
            "Push".to_string(),
        ];
        let ret = build_array(target, n);
        assert_eq!(ret, expected);
    }
    {
        let target = vec![1, 2, 3];
        let n = 3;
        let expected = vec!["Push".to_string(), "Push".to_string(), "Push".to_string()];
        let ret = build_array(target, n);
        assert_eq!(ret, expected);
    }
    {
        let target = vec![1, 2];
        let n = 4;
        let expected = vec!["Push".to_string(), "Push".to_string()];
        let ret = build_array(target, n);
        assert_eq!(ret, expected);
    }
    {
        let target = vec![2, 3, 4];
        let n = 3;
        let expected = vec![
            "Push".to_string(),
            "Pop".to_string(),
            "Push".to_string(),
            "Push".to_string(),
            "Push".to_string(),
        ];
        let ret = build_array(target, n);
        assert_eq!(ret, expected);
    }
}
