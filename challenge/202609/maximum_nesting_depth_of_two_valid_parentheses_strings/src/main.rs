fn max_depth_after_split(seq: String) -> Vec<i32> {
    let mut ret = vec![];
    let mut m = 0;
    for c in seq.chars() {
        if c == '(' {
            m += 1;
            ret.push(m % 2);
        } else {
            ret.push(m % 2);
            m -= 1;
        }
    }
    ret
}

fn main() {
    let ret = max_depth_after_split("(()())".to_string());
    println!("ret={ret:?}");
}

#[test]
fn test() {
    assert_eq!(
        max_depth_after_split("(()())".to_string()),
        vec![1, 0, 0, 0, 0, 1]
    );
    assert_eq!(
        max_depth_after_split("()(())()".to_string()),
        vec![1, 1, 1, 0, 0, 1, 1, 1]
    );
}
