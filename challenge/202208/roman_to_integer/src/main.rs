fn roman_to_int(s: String) -> i32 {
    s.chars()
        .fold((0, ' '), |(acc, prev), c| match c {
            'M' => {
                if prev == 'C' {
                    (acc + 800, c)
                } else {
                    (acc + 1000, c)
                }
            }
            'D' => {
                if prev == 'C' {
                    (acc + 300, c)
                } else {
                    (acc + 500, c)
                }
            }
            'C' => {
                if prev == 'X' {
                    (acc + 80, c)
                } else {
                    (acc + 100, c)
                }
            }
            'L' => {
                if prev == 'X' {
                    (acc + 30, c)
                } else {
                    (acc + 50, c)
                }
            }
            'X' => {
                if prev == 'I' {
                    (acc + 8, c)
                } else {
                    (acc + 10, c)
                }
            }
            'V' => {
                if prev == 'I' {
                    (acc + 3, c)
                } else {
                    (acc + 5, c)
                }
            }
            'I' => (acc + 1, c),
            _ => panic!("never reach here"),
        })
        .0
}

fn main() {
    let ret = roman_to_int("MCMXCIV".to_string());
    println!("ret={ret}");
}

#[test]
fn test_roman_to_int() {
    assert_eq!(roman_to_int("III".to_string()), 3);
    assert_eq!(roman_to_int("LVIII".to_string()), 58);
    assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
}
