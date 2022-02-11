fn check_inclusion(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
        return false;
    }

    let table = s1.bytes().fold(vec![0; 26], |mut acc, b| {
        acc[(b - b'a') as usize] += 1;
        acc
    });

    let bs: Vec<u8> = s2.bytes().map(|b| b - b'a').collect();
    let mut window = vec![0; 26];
    for i in 0..s1.len() {
        window[bs[i] as usize] += 1;
    }

    let check = |window: &Vec<i32>, table: &Vec<i32>| -> bool {
        for i in 0..26 {
            if window[i] != table[i] {
                return false;
            }
        }

        true
    };

    if check(&window, &table) {
        return true;
    }

    for i in s1.len()..s2.len() {
        window[(bs[i - s1.len()]) as usize] -= 1;
        window[(bs[i]) as usize] += 1;

        if check(&window, &table) {
            return true;
        }
    }

    false
}

fn main() {
    let s1 = "ab".to_string();
    let s2 = "eidbaooo".to_string();
    println!("{}", check_inclusion(s1, s2));
}

#[test]
fn test_check_inclusion() {
    {
        let s1 = "ab".to_string();
        let s2 = "eidbaooo".to_string();
        assert!(check_inclusion(s1, s2));
    }
    {
        let s1 = "ab".to_string();
        let s2 = "eidboaoo".to_string();
        assert!(!check_inclusion(s1, s2));
    }
}
