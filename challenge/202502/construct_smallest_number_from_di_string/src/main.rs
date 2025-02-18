fn smallest_number(pattern: String) -> String {
    let mut ret = String::new();
    let mut stack = vec![];

    for (i, c) in pattern.chars().enumerate() {
        stack.push(i as u32 + 1);

        if c == 'I' {
            while let Some(d) = stack.pop() {
                ret.push(std::char::from_digit(d, 10).unwrap());
            }
        }
    }

    stack.push(pattern.len() as u32 + 1);
    while let Some(d) = stack.pop() {
        ret.push(std::char::from_digit(d, 10).unwrap());
    }

    ret
}

fn main() {
    let pattern = "IIIDIDDD".to_string();
    let ret = smallest_number(pattern);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let pattern = "IIIDIDDD".to_string();
        let ret = smallest_number(pattern);
        assert_eq!(ret, "123549876")
    }
    {
        let pattern = "DDD".to_string();
        let ret = smallest_number(pattern);
        assert_eq!(ret, "4321")
    }
}
