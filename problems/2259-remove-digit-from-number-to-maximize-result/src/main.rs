fn remove_digit(number: String, digit: char) -> String {
    let mut ret = vec![];
    for (i, c) in number.chars().enumerate() {
        if c == digit {
            let beg = &number[..i];
            let end = &number[i + 1..];
            ret.push(format!("{beg}{end}"));
        }
    }

    ret.sort_unstable();
    ret.last().unwrap().clone()
}

fn main() {
    let number = "123".to_string();
    let digit = '3';
    let ret = remove_digit(number, digit);
    println!("ret={ret}");
}

#[test]
fn test_remove_digit() {
    {
        let number = "123".to_string();
        let digit = '3';
        let ret = remove_digit(number, digit);
        assert_eq!(ret, "12".to_string());
    }
    {
        let number = "1231".to_string();
        let digit = '1';
        let ret = remove_digit(number, digit);
        assert_eq!(ret, "231".to_string());
    }
    {
        let number = "4234".to_string();
        let digit = '4';
        let ret = remove_digit(number, digit);
        assert_eq!(ret, "423".to_string());
    }
    {
        let number = "551".to_string();
        let digit = '5';
        let ret = remove_digit(number, digit);
        assert_eq!(ret, "51".to_string());
    }
}
