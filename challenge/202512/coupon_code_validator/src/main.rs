fn validate_coupons(
    code: Vec<String>,
    business_line: Vec<String>,
    is_active: Vec<bool>,
) -> Vec<String> {
    fn is_valid1(s: &str) -> bool {
        !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
    }

    fn is_valid2(s: &str) -> bool {
        s == "electronics" || s == "grocery" || s == "pharmacy" || s == "restaurant"
    }

    let mut ret: Vec<_> = code
        .into_iter()
        .enumerate()
        .filter(|(i, c)| is_valid1(c) && is_valid2(&business_line[*i]) && is_active[*i])
        .collect();

    ret.sort_unstable_by(|(i, c1), (j, c2)| {
        if business_line[*i] == business_line[*j] {
            c1.cmp(c2)
        } else {
            business_line[*i].cmp(&business_line[*j])
        }
    });

    ret.into_iter().map(|(_, c)| c).collect()
}

fn main() {
    let code = vec![
        "SAVE20".to_string(),
        "".to_string(),
        "PHARMA5".to_string(),
        "SAVE@20".to_string(),
    ];
    let business_line = vec![
        "restaurant".to_string(),
        "grocery".to_string(),
        "pharmacy".to_string(),
        "restaurant".to_string(),
    ];
    let is_active = vec![true, true, true, true];
    let ret = validate_coupons(code, business_line, is_active);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let code = vec![
            "SAVE20".to_string(),
            "".to_string(),
            "PHARMA5".to_string(),
            "SAVE@20".to_string(),
        ];
        let business_line = vec![
            "restaurant".to_string(),
            "grocery".to_string(),
            "pharmacy".to_string(),
            "restaurant".to_string(),
        ];
        let is_active = vec![true, true, true, true];
        let ret = validate_coupons(code, business_line, is_active);
        assert_eq!(ret, vec!["PHARMA5".to_string(), "SAVE20".to_string()]);
    }

    {
        let code = vec![
            "GROCERY15".to_string(),
            "ELECTRONIC_50".to_string(),
            "DISCOUNT10".to_string(),
        ];
        let business_line = vec![
            "grocery".to_string(),
            "electronics".to_string(),
            "invalid".to_string(),
        ];
        let is_active = vec![false, true, true];
        let ret = validate_coupons(code, business_line, is_active);
        assert_eq!(ret, vec!["ELECTRONIC_50".to_string()]);
    }
}
