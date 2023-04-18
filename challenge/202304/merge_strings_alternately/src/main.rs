fn merge_alternately(word1: String, word2: String) -> String {
    let cs1: Vec<char> = word1.chars().collect();
    let cs2: Vec<char> = word2.chars().collect();

    let mut ret = String::new();
    let mut i = 0usize;
    let mut j = 0usize;

    let mut even = true;
    while i < cs1.len() || j < cs2.len() {
        if i >= cs1.len() {
            ret.push(cs2[j]);
            j += 1;
        } else if j >= cs2.len() {
            ret.push(cs1[i]);
            i += 1;
        } else {
            if even {
                ret.push(cs1[i]);
                i += 1;
            } else {
                ret.push(cs2[j]);
                j += 1;
            }
            even = !even;
        }
    }

    ret
}

fn main() {
    let word1 = "abc".to_string();
    let word2 = "pqr".to_string();
    let ret = merge_alternately(word1, word2);
    println!("ret={ret}");
}

#[test]
fn test_merge_alternately() {
    {
        let word1 = "abc".to_string();
        let word2 = "pqr".to_string();
        let ret = merge_alternately(word1, word2);
        assert_eq!(ret, "apbqcr");
    }
    {
        let word1 = "ab".to_string();
        let word2 = "pqrs".to_string();
        let ret = merge_alternately(word1, word2);
        assert_eq!(ret, "apbqrs");
    }
    {
        let word1 = "abcd".to_string();
        let word2 = "pq".to_string();
        let ret = merge_alternately(word1, word2);
        assert_eq!(ret, "apbqcd");
    }
}
