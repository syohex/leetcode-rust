fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut sv = vec![0; 26];
    let mut tv = vec![0; 26];

    for (a, b) in s.bytes().zip(t.bytes()) {
        sv[(a - b'a') as usize] += 1;
        tv[(b - b'a') as usize] += 1;
    }

    sv.into_iter().zip(tv.into_iter()).all(|(a, b)| a == b)
}

fn main() {
    let s = "anagram".to_string();
    let t = "nagaram".to_string();
    let ret = is_anagram(s, t);
    println!("ret={ret}");
}

#[test]
fn test_is_anagram() {
    {
        let s = "anagram".to_string();
        let t = "nagaram".to_string();
        let ret = is_anagram(s, t);
        assert!(ret);
    }
    {
        let s = "rat".to_string();
        let t = "car".to_string();
        let ret = is_anagram(s, t);
        assert!(!ret);
    }
}
