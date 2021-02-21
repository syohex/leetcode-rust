fn merge_alternately(word1: String, word2: String) -> String {
    let mut pos1 = 0_usize;
    let mut pos2 = 0_usize;

    let mut ret = String::new();
    let cs1: Vec<char> = word1.chars().collect();
    let cs2: Vec<char> = word2.chars().collect();
    while pos1 < cs1.len() || pos2 < cs2.len() {
        if pos1 < cs1.len() {
            ret.push(cs1[pos1]);
            pos1 += 1;
        }
        if pos2 < cs2.len() {
            ret.push(cs2[pos2]);
            pos2 += 1;
        }
    }
    ret
}

fn main() {
    let ret = merge_alternately("abc".to_string(), "pqr".to_string());
    println!("ret={}", ret);
}

#[test]
fn test_merge_alternately() {
    assert_eq!(
        merge_alternately("abc".to_string(), "pqr".to_string()),
        "apbqcr".to_string()
    );
    assert_eq!(
        merge_alternately("ab".to_string(), "pqrs".to_string()),
        "apbqrs".to_string()
    );
    assert_eq!(
        merge_alternately("abcd".to_string(), "pq".to_string()),
        "apbqcd".to_string()
    );
}
