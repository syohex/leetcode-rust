fn shortest_beautiful_substring(s: String, k: i32) -> String {
    let k = k as usize;
    if s.chars().filter(|&c| c == '1').count() < k {
        return String::new();
    }

    let cs: Vec<char> = s.chars().collect();
    let mut left = 0;
    let mut ones = 0;
    let mut ret = s.as_str();
    for (i, c) in s.char_indices() {
        ones += if c == '1' { 1 } else { 0 };
        while ones > k || cs[left] == '0' {
            ones -= if cs[left] == '1' { 1 } else { 0 };
            left += 1;
        }

        if ones == k {
            let len = i - left + 1;
            let substr = &s[left..=i];
            if len < ret.len() || (len == ret.len() && substr < ret) {
                ret = substr;
            }
        }
    }

    ret.to_string()
}

fn main() {
    let ret = shortest_beautiful_substring("100011001".to_string(), 3);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(
        shortest_beautiful_substring("100011001".to_string(), 3),
        "11001"
    );
    assert_eq!(shortest_beautiful_substring("1011".to_string(), 2), "11");
    assert_eq!(shortest_beautiful_substring("000".to_string(), 1), "");
}
