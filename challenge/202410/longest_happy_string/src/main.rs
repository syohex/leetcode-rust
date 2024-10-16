fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
    use std::collections::BinaryHeap;

    let mut q: BinaryHeap<(i32, char)> = BinaryHeap::new();
    if a > 0 {
        q.push((a, 'a'));
    }
    if b > 0 {
        q.push((b, 'b'));
    }
    if c > 0 {
        q.push((c, 'c'));
    }

    let mut ret = String::new();
    let mut prev1 = ' ';
    let mut prev2 = ' ';

    while !q.is_empty() {
        let (count, ch) = q.pop().unwrap();
        if ret.len() >= 2 && prev2 == ch && prev1 == ch {
            if q.is_empty() {
                break;
            }

            let (count2, ch2) = q.pop().unwrap();
            ret.push(ch2);
            if count2 >= 2 {
                q.push((count2 - 1, ch2));
            }

            prev2 = prev1;
            prev1 = ch2;

            q.push((count, ch));
        } else {
            ret.push(ch);
            if count >= 2 {
                q.push((count - 1, ch));
            }

            prev2 = prev1;
            prev1 = ch;
        }
    }

    ret
}

fn main() {
    let ret = longest_diverse_string(1, 1, 7);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(longest_diverse_string(1, 1, 7), "ccbccacc");
    assert_eq!(longest_diverse_string(7, 1, 0), "aabaa");
}
