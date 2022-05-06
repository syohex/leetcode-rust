fn remove_duplicates(s: String, k: i32) -> String {
    let mut stack = vec![];

    for c in s.chars() {
        if let Some((ch, count)) = stack.last_mut() {
            if *ch == c {
                *count += 1;
                if *count == k {
                    stack.pop();
                }
            } else {
                if let Some((ch2, count2)) = stack.last_mut() {
                    if *ch2 == c {
                        *count2 += 1;
                        if *count2 == k {
                            stack.pop();
                        } else if *count2 > k {
                            *count2 -= k;
                        }
                    } else {
                        stack.push((c, 1));
                    }
                } else {
                    stack.push((c, 1));
                }
            }
        } else {
            stack.push((c, 1));
        }
    }

    let mut ret = String::new();
    for (c, count) in stack {
        for _ in 0..count {
            ret.push(c);
        }
    }

    ret
}

fn main() {
    let s = "pbbcggttciiippooaais".to_string();
    let ret = remove_duplicates(s, 2);
    println!("ret={ret}");
}

#[test]
fn test_remove_duplicates() {
    {
        let s = "abcd".to_string();
        let expected = "abcd".to_string();
        let ret = remove_duplicates(s, 2);
        assert_eq!(ret, expected);
    }
    {
        let s = "deeedbbcccbdaa".to_string();
        let expected = "aa".to_string();
        let ret = remove_duplicates(s, 3);
        assert_eq!(ret, expected);
    }
    {
        let s = "pbbcggttciiippooaais".to_string();
        let expected = "ps".to_string();
        let ret = remove_duplicates(s, 2);
        assert_eq!(ret, expected);
    }
    {
        let s = "yfttttfbbbbnnnnffbgffffgbbbbgssssgthyyyy".to_string();
        let expected = "ybth".to_string();
        let ret = remove_duplicates(s, 4);
        assert_eq!(ret, expected);
    }
}
