fn find_mismatched_segments(s1: String, s2: String) -> Vec<Vec<String>> {
    let mut ret = vec![];
    let cs1: Vec<_> = s1.chars().collect();
    let cs2: Vec<_> = s2.chars().collect();

    let mut start = usize::MAX;
    for (i, (c1, c2)) in cs1.iter().zip(cs2.iter()).enumerate() {
        if *c1 != *c2 {
            if start == usize::MAX {
                start = i;
            }

            if i == cs1.len() - 1 {
                ret.push(vec![
                    cs1.iter().take(i + 1).skip(start).collect(),
                    cs2.iter().take(i + 1).skip(start).collect(),
                ]);
            }
        } else if start != usize::MAX {
            ret.push(vec![
                cs1.iter().take(i).skip(start).collect(),
                cs2.iter().take(i).skip(start).collect(),
            ]);
            start = usize::MAX;
        }
    }

    ret
}

fn main() {
    let s1 = "earth".to_string();
    let s2 = "empty".to_string();
    let ret = find_mismatched_segments(s1, s2);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let s1 = "earth".to_string();
        let s2 = "empty".to_string();
        let expected = vec![
            vec!["ar".to_string(), "mp".to_string()],
            vec!["h".to_string(), "y".to_string()],
        ];
        let ret = find_mismatched_segments(s1, s2);
        assert_eq!(ret, expected);
    }
    {
        let s1 = "action".to_string();
        let s2 = "nation".to_string();
        let expected = vec![vec!["ac".to_string(), "na".to_string()]];
        let ret = find_mismatched_segments(s1, s2);
        assert_eq!(ret, expected);
    }
    {
        let s1 = "hot".to_string();
        let s2 = "hot".to_string();
        let ret = find_mismatched_segments(s1, s2);
        assert!(ret.is_empty());
    }
}
