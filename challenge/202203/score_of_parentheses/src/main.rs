fn score_of_parentheses(s: String) -> i32 {
    fn tmp(s: &Vec<char>, idx: usize) -> (usize, i32) {
        let mut i = idx;

        let mut ret = 0;
        while i < s.len() {
            if s[i] == '(' {
                let (j, score) = tmp(s, i + 1);
                i = j;
                ret += score;
            } else {
                if ret == 0 {
                    return (i + 1, 1);
                } else {
                    return (i + 1, ret * 2);
                }
            }
        }

        (s.len(), ret)
    }

    let cs = s.chars().collect();
    tmp(&cs, 0).1
}

fn main() {
    let ret = score_of_parentheses("((()))".to_string());
    println!("ret={ret}");
}

#[test]
fn test_score_of_parentheses() {
    assert_eq!(score_of_parentheses("()".to_string()), 1);
    assert_eq!(score_of_parentheses("((()))".to_string()), 4);
    assert_eq!(score_of_parentheses("()()()".to_string()), 3);
    assert_eq!(score_of_parentheses("(()(()))".to_string()), 6);
}
