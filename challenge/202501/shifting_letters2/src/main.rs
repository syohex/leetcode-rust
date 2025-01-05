fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
    let mut merged = vec![0i32; s.len()];
    for shift in shifts {
        if shift[2] == 1 {
            merged[shift[0] as usize] += 1;
            if shift[1] as usize + 1 < s.len() {
                merged[shift[1] as usize + 1] -= 1;
            }
        } else {
            merged[shift[0] as usize] -= 1;
            if shift[1] as usize + 1 < s.len() {
                merged[shift[1] as usize + 1] += 1;
            }
        }
    }

    let mut ret = String::new();
    let mut shift_size = 0;
    for (i, b) in s.bytes().enumerate() {
        shift_size += merged[i];

        let mut shifted = ((b - b'a') as i32 + shift_size) % 26;
        if shifted < 0 {
            shifted += 26;
        }

        ret.push((shifted as u8 + b'a') as char);
    }

    ret
}

fn main() {
    let s = "abc".to_string();
    let shifts = vec![vec![0, 1, 0], vec![1, 2, 1], vec![0, 2, 1]];
    let ret = shifting_letters(s, shifts);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let s = "abc".to_string();
        let shifts = vec![vec![0, 1, 0], vec![1, 2, 1], vec![0, 2, 1]];
        let ret = shifting_letters(s, shifts);
        assert_eq!(ret, "ace");
    }
    {
        let s = "dztz".to_string();
        let shifts = vec![vec![0, 0, 0], vec![1, 1, 1]];
        let ret = shifting_letters(s, shifts);
        assert_eq!(ret, "catz");
    }
}
