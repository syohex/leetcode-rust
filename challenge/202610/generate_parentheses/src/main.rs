fn generate_parenthesis(n: i32) -> Vec<String> {
    fn f(n: i32, opens: i32, s: &mut String, acc: &mut Vec<String>) {
        if n == 0 && opens == 0 {
            acc.push(s.clone());
            return;
        }

        if opens >= 1 {
            s.push(')');
            f(n, opens - 1, s, acc);
            s.pop();
        }

        if n >= 1 {
            s.push('(');
            f(n - 1, opens + 1, s, acc);
            s.pop();
        }
    }

    let mut acc = vec![];
    let mut tmp = String::new();
    f(n, 0, &mut tmp, &mut acc);
    acc
}
fn main() {
    let ret = generate_parenthesis(3);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    use std::collections::HashSet;
    fn eq(a: Vec<String>, b: Vec<String>) {
        let a: HashSet<_> = a.into_iter().collect();
        let b: HashSet<_> = b.into_iter().collect();
        assert_eq!(a, b);
    }

    {
        let expected = vec![
            "((()))".to_string(),
            "(()())".to_string(),
            "(())()".to_string(),
            "()(())".to_string(),
            "()()()".to_string(),
        ];
        let ret = generate_parenthesis(3);
        eq(ret, expected);
    }
    {
        let expected = vec!["()".to_string()];
        let ret = generate_parenthesis(1);
        eq(ret, expected);
    }
}
