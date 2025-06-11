fn find_valid_pair(s: String) -> String {
    let bs : Vec<_> = s.bytes().collect();
    let mut freq = [0;9];
    for &b in &bs {
        freq[(b - b'1') as usize] += 1;
    }

    for (i, w) in bs.windows(2).enumerate() {
        if w[0] == w[1] {
            continue;
        }

        let count1 = freq[(w[0] - b'1') as usize];
        let count2 = freq[(w[1] - b'1') as usize];

        if count1 == w[0] - b'0' && count2 == w[1] - b'0' {
            return s[i..(i+2)].to_string();
        }
    }

    "".to_string()
}

fn main() {
    let ret = find_valid_pair("2523533".to_string());
    println!("ret={ret}");
}

#[test]
fn test()
{
    assert_eq!(find_valid_pair("2523533".to_string()), "23");
    assert_eq!(find_valid_pair("221".to_string()), "21");
    assert_eq!(find_valid_pair("22".to_string()), "");
}
