fn score_validator(events: Vec<String>) -> Vec<i32> {
    let mut score = 0;
    let mut counter = 0;

    for e in events {
        match e.as_str() {
            "W" => {
                counter += 1;
            }
            "WD" | "NB" => {
                score += 1;
            }
            n => {
                let v: i32 = n.parse().unwrap();
                score += v;
            }
        }
        if counter >= 10 {
            break;
        }
    }

    vec![score, counter]
}
fn main() {
    let ret = score_validator(vec![
        "1".to_string(),
        "4".to_string(),
        "W".to_string(),
        "6".to_string(),
        "WD".to_string(),
    ]);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    assert_eq!(
        score_validator(vec![
            "1".to_string(),
            "4".to_string(),
            "W".to_string(),
            "6".to_string(),
            "WD".to_string()
        ]),
        vec![12, 1]
    );
    assert_eq!(
        score_validator(vec![
            "WD".to_string(),
            "NB".to_string(),
            "0".to_string(),
            "4".to_string(),
            "4".to_string()
        ]),
        vec![10, 0]
    );
    assert_eq!(
        score_validator(vec![
            "W".to_string(),
            "W".to_string(),
            "W".to_string(),
            "W".to_string(),
            "W".to_string(),
            "W".to_string(),
            "W".to_string(),
            "W".to_string(),
            "W".to_string(),
            "W".to_string(),
            "W".to_string()
        ]),
        vec![0, 10]
    );
}
