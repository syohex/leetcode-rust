fn find_common_reponse(responses: Vec<Vec<String>>) -> String {
    use std::collections::{HashMap, HashSet};

    responses
        .into_iter()
        .map(|v| v.into_iter().collect::<HashSet<_>>())
        .fold(HashMap::new(), |mut acc, s| {
            for w in s {
                *acc.entry(w).or_insert(0) += 1;
            }
            acc
        })
        .into_iter()
        .fold(("".to_string(), 0), |(ret, max), (k, v)| {
            if v > max || (v == max && k < ret) {
                (k, v)
            } else {
                (ret, max)
            }
        })
        .0
}

fn main() {
    let responses = vec![
        vec![
            "good".to_string(),
            "ok".to_string(),
            "good".to_string(),
            "ok".to_string(),
        ],
        vec![
            "ok".to_string(),
            "bad".to_string(),
            "good".to_string(),
            "ok".to_string(),
            "ok".to_string(),
        ],
        vec!["good".to_string()],
        vec!["bad".to_string()],
    ];
    let ret = find_common_reponse(responses);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let responses = vec![
            vec![
                "good".to_string(),
                "ok".to_string(),
                "good".to_string(),
                "ok".to_string(),
            ],
            vec![
                "ok".to_string(),
                "bad".to_string(),
                "good".to_string(),
                "ok".to_string(),
                "ok".to_string(),
            ],
            vec!["good".to_string()],
            vec!["bad".to_string()],
        ];
        let ret = find_common_reponse(responses);
        assert_eq!(ret, "good");
    }
    {
        let responses = vec![
            vec!["good".to_string(), "ok".to_string(), "good".to_string()],
            vec!["ok".to_string(), "bad".to_string()],
            vec!["bad".to_string(), "notsure".to_string()],
            vec!["great".to_string(), "good".to_string()],
        ];
        let ret = find_common_reponse(responses);
        assert_eq!(ret, "bad");
    }
}
