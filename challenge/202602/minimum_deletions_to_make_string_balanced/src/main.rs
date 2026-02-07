fn minimum_deletions(s: String) -> i32 {
    let cs: Vec<_> = s.chars().collect();

    let len = cs.len();
    let mut bv = vec![0; len];
    let mut b_prefix_count = 0;
    for (i, &c) in cs.iter().enumerate() {
        bv[i] = b_prefix_count;
        if c == 'b' {
            b_prefix_count += 1;
        }
    }

    let mut av = vec![0; len];
    let mut a_postfix_count = 0;
    for (i, &c) in cs.iter().enumerate().rev() {
        av[i] = a_postfix_count;
        if c == 'a' {
            a_postfix_count += 1;
        }
    }

    av.into_iter()
        .zip(bv)
        .fold(i32::MAX, |acc, (a, b)| std::cmp::min(acc, a + b))
}

fn main() {
    let ret = minimum_deletions("aababbab".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(minimum_deletions("aababbab".to_string()), 2);
    assert_eq!(minimum_deletions("bbaaaaabb".to_string()), 2);
}
