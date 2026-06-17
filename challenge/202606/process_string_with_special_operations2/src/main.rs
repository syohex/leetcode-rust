fn process_str(s: String, k: i64) -> char {
    let mut len = 0;
    for c in s.chars() {
        match c {
            '*' => {
                if len > 0 {
                    len -= 1;
                }
            }
            '#' => len *= 2,
            '%' => (),
            _ => len += 1,
        }
    }

    if k + 1 > len {
        return '.';
    }

    let mut k = k;
    for c in s.chars().rev() {
        match c {
            '*' => len += 1,
            '#' => {
                if k + 1 > (len + 1) / 2 {
                    k -= len / 2;
                }
                len = (len + 1) / 2
            }
            '%' => {
                k = len - k - 1;
            }
            _ => {
                if k + 1 == len {
                    return c;
                }

                len -= 1;
            }
        }
    }

    '.'
}

fn main() {
    let ret = process_str("a#b%*".to_string(), 1);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(process_str("a#b%*".to_string(), 1), 'a');
    assert_eq!(process_str("cd%#*#".to_string(), 3), 'd');
    assert_eq!(process_str("z*#".to_string(), 0), '.');
}
