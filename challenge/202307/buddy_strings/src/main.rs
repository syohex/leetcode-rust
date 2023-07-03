fn buddy_strings(s: String, goal: String) -> bool {
    if s.len() != goal.len() {
        return false;
    }

    let mut diffs = 0;
    let mut v1 = vec![0; 26];
    let mut v2 = vec![0; 26];

    let mut diff_pairs = vec![];
    for (a, b) in s.bytes().zip(goal.bytes()) {
        if a != b {
            diffs += 1;
            diff_pairs.push((a, b));
        }

        v1[(a - b'a') as usize] += 1;
        v2[(b - b'a') as usize] += 1;
    }

    if v1 != v2 {
        return false;
    }

    if diffs >= 3 || diffs == 1 {
        false
    } else if diffs == 0 {
        v1.into_iter().any(|c| c >= 2)
    } else {
        diff_pairs[0].0 == diff_pairs[1].1 && diff_pairs[0].1 == diff_pairs[1].0
    }
}

fn main() {
    let s = "ab".to_string();
    let goal = "ba".to_string();
    let ret = buddy_strings(s, goal);
    println!("ret={ret}");
}

#[test]
fn test_buddy_strings() {
    {
        let s = "ab".to_string();
        let goal = "ba".to_string();
        let ret = buddy_strings(s, goal);
        assert!(ret);
    }
    {
        let s = "ab".to_string();
        let goal = "ab".to_string();
        let ret = buddy_strings(s, goal);
        assert!(!ret);
    }
    {
        let s = "aa".to_string();
        let goal = "aa".to_string();
        let ret = buddy_strings(s, goal);
        assert!(ret);
    }
    {
        let s = "ab".to_string();
        let goal = "ca".to_string();
        let ret = buddy_strings(s, goal);
        assert!(!ret);
    }
    {
        let s = "aaa".to_string();
        let goal = "aab".to_string();
        let ret = buddy_strings(s, goal);
        assert!(!ret);
    }
}
