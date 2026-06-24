fn create_grid(m: i32, n: i32) -> Vec<String> {
    let (m, n) = (m as usize, n as usize);
    let mut last = String::new();
    for _ in 0..n {
        last.push('.');
    }

    let mut ret = vec![];
    for _ in 0..(m - 1) {
        let mut s = String::new();
        s.push('.');
        for _ in 1..n {
            s.push('#');
        }
        ret.push(s);
    }
    ret.push(last);

    ret
}

fn main() {
    let ret = create_grid(3, 3);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let expected = vec![".##".to_string(), "...".to_string()];
        let ret = create_grid(2, 3);
        assert_eq!(ret, expected);
    }
    {
        let expected = vec![".##".to_string(), ".##".to_string(), "...".to_string()];
        let ret = create_grid(3, 3);
        assert_eq!(ret, expected);
    }
    {
        let expected = vec!["....".to_string()];
        let ret = create_grid(1, 4);
        assert_eq!(ret, expected);
    }
    {
        let expected = vec![".".to_string(), ".".to_string(), ".".to_string()];
        let ret = create_grid(3, 1);
        assert_eq!(ret, expected);
    }
}
