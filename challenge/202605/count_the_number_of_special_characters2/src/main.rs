fn number_of_special_chars(word: String) -> i32 {
    let mut lowers = [false; 26];
    let mut uppers = [false; 26];
    let mut checked = [false; 26];

    let mut ret = 0;
    for b in word.bytes() {
        let is_lower = b.is_ascii_lowercase();
        let idx = if is_lower {
            (b - b'a') as usize
        } else {
            (b - b'A') as usize
        };
        if checked[idx] {
            continue;
        }

         if is_lower {
            if uppers[idx] {
                checked[idx] = true;
                if lowers[idx] {
                    ret -= 1;
                }
            }

            lowers[idx] = true;
        } else {
            if lowers[idx] && !uppers[idx] {
                ret += 1;
            }
            uppers[idx] = true;
        }
    }

    ret
}

fn main() {
    let ret = number_of_special_chars("aaAbcBC".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(number_of_special_chars("cADEDee".to_string()), 0);
    assert_eq!(number_of_special_chars("dcbCC".to_string()), 1);
    assert_eq!(number_of_special_chars("aaAbcBC".to_string()), 3);
    assert_eq!(number_of_special_chars("abc".to_string()), 0);
    assert_eq!(number_of_special_chars("AbBCab".to_string()), 0);
}
