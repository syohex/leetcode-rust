fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    use std::collections::HashMap;
    use std::collections::HashSet;

    let mut h: HashMap<i32, HashSet<i32>> = HashMap::new();
    for log in &logs {
        if let Some(v) = h.get_mut(&log[0]) {
            v.insert(log[1]);
        } else {
            h.insert(log[0], HashSet::from([log[1]]));
        }
    }

    h.values().fold(vec![0; k as usize], |mut acc, v| {
        acc[v.len() - 1] += 1;
        acc
    })
}

fn main() {
    let logs = vec![vec![0, 5], vec![1, 2], vec![0, 2], vec![0, 5], vec![1, 3]];
    let ret = finding_users_active_minutes(logs, 5);
    println!("ret={:?}", ret);
}

#[test]
fn test_finding_user_active_minutes() {
    {
        let logs = vec![vec![0, 5], vec![1, 2], vec![0, 2], vec![0, 5], vec![1, 3]];
        let expected = vec![0, 2, 0, 0, 0];
        let ret = finding_users_active_minutes(logs, 5);
        assert_eq!(ret, expected);
    }
    {
        let logs = vec![vec![1, 1], vec![2, 2], vec![2, 3]];
        let expected = vec![1, 1, 0, 0];
        let ret = finding_users_active_minutes(logs, 4);
        assert_eq!(ret, expected);
    }
}
