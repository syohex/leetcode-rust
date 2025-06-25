fn generate_tag(caption: String) -> String {
    let mut ret = String::new();
    ret.push('#');

    let mut is_first = true;
    let mut is_capital = true;
    for c in caption.chars() {
        if c == ' ' {
            is_capital = true;
            continue;
        }

        if ret.len() >= 100 {
            break;
        }

        if is_first {
            ret.push(c.to_ascii_lowercase());
            is_first = false;
            is_capital = false;
        } else if is_capital {
            is_capital = false;
            ret.push(c.to_ascii_uppercase());
        } else {
            ret.push(c.to_ascii_lowercase());
        }
    }

    ret
}

fn main() {
    let caption = "Leetcode daily streak archived".to_string();
    let ret = generate_tag(caption);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let caption = "Leetcode daily streak achieved".to_string();
        let expected = "#leetcodeDailyStreakAchieved";
        let ret = generate_tag(caption);
        assert_eq!(ret, expected);
    }
    {
        let caption = "can I Go There".to_string();
        let expected = "#canIGoThere";
        let ret = generate_tag(caption);
        assert_eq!(ret, expected);
    }
    {
        let mut caption = String::new();
        for _ in 0..200 {
            caption.push('a');
        }
        let ret = generate_tag(caption);
        assert_eq!(ret.len(), 100);
    }
}
