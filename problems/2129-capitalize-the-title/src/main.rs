fn capitalized_title(title: String) -> String {
    let mut ret = String::new();

    for (i, s) in title.split(' ').enumerate() {
        if i != 0 {
            ret.push(' ');
        }

        if s.len() <= 2 {
            ret.push_str(&s.to_ascii_lowercase());
        } else {
            for (j, c) in s.chars().enumerate() {
                if j == 0 {
                    ret.push(c.to_ascii_uppercase());
                } else {
                    ret.push(c.to_ascii_lowercase());
                }
            }
        }
    }

    ret
}

fn main() {
    let ret = capitalized_title("First leTTeR of EACH Word".to_string());
    println!("ret={}", ret);
}

#[test]
fn test_capitalize_title() {
    {
        let title = "capiTalIze tHe title".to_string();
        assert_eq!(capitalized_title(title), "Capitalize The Title".to_string());
    }
    {
        let title = "First leTTeR of EACH Word".to_string();
        assert_eq!(
            capitalized_title(title),
            "First Letter of Each Word".to_string()
        );
    }
    {
        let title = "i lOve leetcode".to_string();
        assert_eq!(capitalized_title(title), "i Love Leetcode".to_string());
    }
}
