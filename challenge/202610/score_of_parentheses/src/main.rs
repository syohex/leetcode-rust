fn score_of_parentheses(s: String) -> i32 {
    fn f(mut pos: usize, cs: &[char]) -> (i32, usize) {
        if pos >= cs.len() {
            return (0, pos);
        }

        let mut score = 0;
        while pos < cs.len() {
            if cs[pos] == '(' {
                let (s, p) = f(pos + 1, cs);
                pos = p;
                score += s;
            } else if score == 0 {
                return (1, pos + 1);
            } else {
                return (score * 2, pos + 1);
            }
        }

        (score, pos)
    }

    let cs: Vec<_> = s.chars().collect();
    f(0, &cs).0
}

fn main() {
    let ret = score_of_parentheses("(((())))".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(score_of_parentheses("(()(()))".to_string()), 6);
    assert_eq!(score_of_parentheses("()".to_string()), 1);
    assert_eq!(score_of_parentheses("(())".to_string()), 2);
    assert_eq!(score_of_parentheses("()()".to_string()), 2);
}
