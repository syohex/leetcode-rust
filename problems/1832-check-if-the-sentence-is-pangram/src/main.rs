fn check_if_pangram(sentence: String) -> bool {
    let mut table = vec![0; 26];
    for b in sentence.bytes() {
        let index = (b - b'a') as usize;
        table[index] += 1;
    }

    table.iter().all(|n| *n != 0)
}

fn main() {
    let sentence = "thequickbrownfoxjumpsoverthelazydog";
    println!("ret={}", check_if_pangram(sentence.to_string()));
}

#[test]
fn test_check_if_pangram() {
    {
        let sentence = "thequickbrownfoxjumpsoverthelazydog";
        assert!(check_if_pangram(sentence.to_string()))
    }
    {
        let sentence = "leetcode";
        assert!(!check_if_pangram(sentence.to_string()))
    }
}
