fn get_happy_string(n: i32, k: i32) -> String {
    fn f(pos: usize, prev: char, n: usize, acc: &mut String, ret:&mut Vec<String>) {
        if pos == n {
            ret.push(acc.clone());
            return;
        }

        for c in "abc".chars() {
            if c != prev {
                acc.push(c);
                f(pos + 1, c, n, acc, ret);
                acc.pop();
            }
        }
    }

    let k = k as usize;
    let mut acc = String::new();
    let mut ret = vec![];
    f(0, '?', n as usize, &mut acc, &mut ret);

    if k <= ret.len() {
        ret[k - 1].clone()
    } else {
        "".to_string()
    }
}

fn main() {
    let ret = get_happy_string(10, 100);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(get_happy_string(1, 3), "c");
    assert_eq!(get_happy_string(1, 4), "");
    assert_eq!(get_happy_string(3, 9), "cab");
}
