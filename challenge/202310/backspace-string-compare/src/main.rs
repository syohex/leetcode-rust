fn backspace_compare(s: String, t: String) -> bool {
    let mut i = s.len() as i32 - 1;
    let mut j = t.len() as i32 - 1;

    let sv: Vec<char> = s.chars().collect();
    let tv: Vec<char> = t.chars().collect();

    while i >= 0 || j >= 0 {
        let mut skips = 0;
        while i >= 0 {
            if sv[i as usize] == '#' {
                skips += 1;
                i -= 1;
            } else if skips > 0 {
                skips -= 1;
                i -= 1;
            } else {
                break;
            }
        }

        skips = 0;
        while j >= 0 {
            if tv[j as usize] == '#' {
                skips += 1;
                j -= 1;
            } else if skips > 0 {
                skips -= 1;
                j -= 1;
            } else {
                break;
            }
        }

        if i < 0 && j < 0 {
            return true;
        } else if i < 0 || j < 0 {
            return false;
        }

        if sv[i as usize] != tv[j as usize] {
            return false;
        }

        i -= 1;
        j -= 1;
    }

    true
}

fn main() {
    let s = "ab#c".to_string();
    let t = "ad#c".to_string();
    let ret = backspace_compare(s, t);
    println!("ret={ret}");
}

#[test]
fn test_backspace_compare() {
    {
        let s = "ab#c".to_string();
        let t = "ad#c".to_string();
        let ret = backspace_compare(s, t);
        assert!(ret);
    }
    {
        let s = "ab##".to_string();
        let t = "c#d#".to_string();
        let ret = backspace_compare(s, t);
        assert!(ret);
    }
    {
        let s = "a#c".to_string();
        let t = "b".to_string();
        let ret = backspace_compare(s, t);
        assert!(!ret);
    }
    {
        let s = "bbbextm".to_string();
        let t = "bbb#extm".to_string();
        let ret = backspace_compare(s, t);
        assert!(!ret);
    }
    {
        let s = "nzp#o#g".to_string();
        let t = "b#nzp#o#g".to_string();
        let ret = backspace_compare(s, t);
        assert!(ret);
    }
}
