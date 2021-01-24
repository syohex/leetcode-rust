fn maximum_time(time: String) -> String {
    let mut cs: Vec<char> = time.chars().collect();
    if cs[0] == '?' {
        cs[0] = if cs[1] >= '4' && cs[1] <= '9' {
            '1'
        } else {
            '2'
        }
    }
    if cs[1] == '?' {
        cs[1] = if cs[0] == '2' { '3' } else { '9' }
    }
    if cs[3] == '?' {
        cs[3] = '5';
    }
    if cs[4] == '?' {
        cs[4] = '9'
    }

    cs.into_iter().collect()
}

fn main() {
    println!(
        "maximum_time('2?:?0')={}",
        maximum_time("2?:?0".to_string())
    );
}

#[test]
fn test_maximum_time() {
    assert_eq!(maximum_time("2?:?0".to_string()), "23:50".to_string());
    assert_eq!(maximum_time("0?:3?".to_string()), "09:39".to_string());
    assert_eq!(maximum_time("1?:22".to_string()), "19:22".to_string());
    assert_eq!(maximum_time("?4:03".to_string()), "14:03".to_string());
}
