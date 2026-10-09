fn min_insertions(s: String) -> i32 {
    use std::collections::VecDeque;

    let mut opens = 0;
    let mut missings = 0;
    let mut q : VecDeque<_> = s.chars().collect();

    while !q.is_empty() {
        let c = q.pop_front().unwrap();
        if c == '(' {
            opens += 1;
        } else {
            if let Some(d) = q.pop_front() {
                if opens >= 1 {
                    if d == ')' {
                        opens -= 1;
                    } else {
                        missings += 1;
                    }
                } else {
                    if d == ')' {
                        missings += 1;
                    } else {
                        missings += 2;
                        opens += 1;
                    }
                }
            } else {
                if opens >= 1 {
                    missings += 1;
                    opens -= 1;
                } else {
                    missings += 2;
                }
            }
        }
    }

    opens * 2 + missings
}

fn main() {
    let ret = min_insertions("(()))".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(min_insertions("))".to_string()), 1);
    assert_eq!(min_insertions(")".to_string()), 2);
    assert_eq!(min_insertions("(()))".to_string()), 1);
    assert_eq!(min_insertions("())".to_string()), 0);
    assert_eq!(min_insertions("))())(".to_string()), 3);
}
