fn process_str(s: String) -> String {
    let mut v: Vec<char> = vec![];
    for c in s.chars() {
        match c {
            '*' => {
                v.pop();
            }
            '#' => {
                let mut tmp = v.clone();
                v.append(&mut tmp);
            }
            '%' => v.reverse(),
            _ => v.push(c),
        }
    }

    v.into_iter().collect()
}

fn main() {
    let ret = process_str("a#b%*".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(process_str("a#b%*".to_string()), "ba");
    assert_eq!(process_str("z*#".to_string()), "");
}
