fn can_be_valid(s: String, locked: String) -> bool {
    if s.len() % 2 != 0 {
        return false;
    }

    let mut opens = vec![];
    let mut zeros = vec![];

    for (i, (c, l)) in s.chars().zip(locked.chars()).enumerate() {
        if l == '0' {
            zeros.push(i);
        } else if c == '(' {
            opens.push(i);
        } else if !opens.is_empty() {
            opens.pop();
        } else if !zeros.is_empty() {
            zeros.pop();
        } else {
            return false;
        }
    }

    while !opens.is_empty() && !zeros.is_empty() {
        let last_open = *opens.last().unwrap();
        let last_zero = *zeros.last().unwrap();

        if last_open < last_zero {
            opens.pop();
            zeros.pop();
        } else {
            return false;
        }
    }

    opens.is_empty()
}

fn main() {
    let s = "))()))".to_string();
    let locked = "010100".to_string();
    let ret = can_be_valid(s, locked);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let s = "))()))".to_string();
        let locked = "010100".to_string();
        assert!(can_be_valid(s, locked));
    }
    {
        let s = "()()".to_string();
        let locked = "0000".to_string();
        assert!(can_be_valid(s, locked));
    }
    {
        let s = ")".to_string();
        let locked = "0".to_string();
        assert!(!can_be_valid(s, locked));
    }
    {
        let s = ")(".to_string();
        let locked = "00".to_string();
        assert!(can_be_valid(s, locked));
    }
    {
        let s = "()".to_string();
        let locked = "11".to_string();
        assert!(can_be_valid(s, locked));
    }
}
