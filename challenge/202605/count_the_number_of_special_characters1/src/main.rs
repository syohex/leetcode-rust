fn number_of_special_characters(word: String) -> i32 {
    let mut uppers = [false; 26];
    let mut lowers = [false; 26];
    let mut checked = [false; 26];
    let mut ret = 0;

    for b in word.bytes() {
        if b.is_ascii_lowercase() {
            let idx = (b - b'a') as usize;
            if uppers[idx] && !checked[idx] {
                ret += 1;
                checked[idx] = true;
            }
            lowers[idx] = true;
        } else {
            let idx = (b - b'A') as usize;
            if lowers[idx] && !checked[idx] {
                ret += 1;
                checked[idx] = true;
            }
            uppers[idx] = true;
        }
    }

    ret
}

fn main() {
    let ret = number_of_special_characters("aaAbcBC".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(number_of_special_characters("aaaaAAAAAAAA".to_string()), 1);
    assert_eq!(number_of_special_characters("aaAbcBC".to_string()), 3);
    assert_eq!(number_of_special_characters("abc".to_string()), 0);
    assert_eq!(number_of_special_characters("abBCab".to_string()), 1);
}
