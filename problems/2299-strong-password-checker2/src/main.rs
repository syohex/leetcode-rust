fn strong_password_checker_ii(password: String) -> bool {
    if password.len() < 8 {
        return false;
    }

    let mut has_lower = false;
    let mut has_upper = false;
    let mut has_digit = false;
    let mut has_special = false;
    let mut prev = 255 as char;
    for c in password.chars() {
        if c.is_ascii_lowercase() {
            has_lower = true;
        } else if c.is_ascii_uppercase() {
            has_upper = true;
        } else if c.is_ascii_digit() {
            has_digit = true;
        } else if "!@#$%^&*()-+".contains(c) {
            has_special = true;
        }

        if prev == c {
            return false;
        }

        prev = c;
    }

    has_lower && has_upper && has_digit && has_special
}

fn main() {
    let password = "IloveLe3tcode!".to_string();
    let ret = strong_password_checker_ii(password);
    println!("ret={ret}");
}

#[test]
fn test_strong_password_checker_ii() {
    {
        let password = "IloveLe3tcode!".to_string();
        assert!(strong_password_checker_ii(password));
    }
    {
        let password = "Me+You--IsMyDream".to_string();
        assert!(!strong_password_checker_ii(password));
    }
    {
        let password = "1aB!".to_string();
        assert!(!strong_password_checker_ii(password));
    }
}
