fn reverse_parentheses(s: String) -> String {
    fn f(mut pos: usize, cs: &[char]) -> (usize, String) {
        let mut tmp = String::new();

        while pos < cs.len() {
            match cs[pos] {
                ')' => {
                    break;
                }
                '(' => {
                    let (new_pos, s) = f(pos + 1, cs);
                    tmp.push_str(s.as_str());
                    pos = new_pos;
                }
                _ => {
                    tmp.push(cs[pos]);
                    pos += 1;
                }
            }
        }

        (pos + 1, tmp.chars().rev().collect())
    }

    let mut ret = String::new();
    let cs: Vec<_> = s.chars().collect();
    let mut pos = 0;
    while pos < cs.len() {
        match cs[pos] {
            '(' => {
                let (new_pos, in_paren) = f(pos + 1, &cs);
                pos = new_pos;
                ret.push_str(in_paren.as_ref());
            }
            c => {
                ret.push(c);
                pos += 1;
            }
        }
    }

    ret
}

fn main() {
    let ret = reverse_parentheses("(a(bc)d)".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(
        reverse_parentheses("a(bcdefghijkl(mno)p)q".to_string()),
        "apmnolkjihgfedcbq"
    );
    assert_eq!(reverse_parentheses("(u(love)i)".to_string()), "iloveu");
    assert_eq!(reverse_parentheses("(abcd)".to_string()), "dcba");
    assert_eq!(
        reverse_parentheses("(ed(et(oc))el)".to_string()),
        "leetcode"
    );
}
