fn roman_to_int(s: String) -> i32 {
    let mut ret = 0;
    let mut pos = 0;
    let cs: Vec<char> = s.chars().collect();

    fn calc(cs: &Vec<char>, pos: &mut usize, high: char, mid: char, low: char, base: i32) -> i32 {
        let mut ret = 0;
        if *pos + 1 < cs.len() && cs[*pos] == low && cs[*pos + 1] == high {
            ret += base * 9;
            *pos += 2;
        }

        if *pos < cs.len() && cs[*pos] == mid {
            ret += base * 5;
            *pos += 1;
        }

        if *pos + 1 < cs.len() && cs[*pos] == low && cs[*pos + 1] == mid {
            ret += base * 4;
            *pos += 2;
        }

        while *pos < cs.len() && cs[*pos] == low {
            ret += base;
            *pos += 1;
        }

        ret
    }

    while pos < cs.len() && cs[pos] == 'M' {
        ret += 1000;
        pos += 1;
    }

    ret += calc(&cs, &mut pos, 'M', 'D', 'C', 100);
    ret += calc(&cs, &mut pos, 'C', 'L', 'X', 10);
    ret += calc(&cs, &mut pos, 'X', 'V', 'I', 1);

    ret
}

fn main() {
    println!("roman_to_int('III')={}", roman_to_int("III".to_string()));
}

#[test]
fn test_roman_to_int() {
    assert_eq!(roman_to_int("III".to_string()), 3);
    assert_eq!(roman_to_int("IV".to_string()), 4);
    assert_eq!(roman_to_int("IX".to_string()), 9);
    assert_eq!(roman_to_int("LVIII".to_string()), 58);
    assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    assert_eq!(roman_to_int("MMM".to_string()), 3000);
}
