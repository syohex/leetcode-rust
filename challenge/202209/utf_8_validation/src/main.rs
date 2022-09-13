fn valid_utf8(data: Vec<i32>) -> bool {
    let len = data.len();
    let data: Vec<u8> = data.into_iter().map(|n| n as u8).collect();
    let mut i = 0usize;

    while i < len {
        if (data[i] & 0b10000000) == 0 {
            i += 1;
        } else if (data[i] & 0b11100000) == 0b11000000 {
            if i + 1 >= len {
                return false;
            }

            if (data[i + 1] & 0b11000000) == 0b10000000 {
                i += 2;
            } else {
                return false;
            }
        } else if (data[i] & 0b11110000) == 0b11100000 {
            if i + 2 >= len {
                return false;
            }

            if (data[i + 1] & 0b11000000) == 0b10000000 && (data[i + 2] & 0b11000000) == 0b10000000
            {
                i += 3;
            } else {
                return false;
            }
        } else if (data[i] & 0b11111000) == 0b11110000 {
            if i + 3 >= len {
                return false;
            }

            if (data[i + 1] & 0b11000000) == 0b10000000
                && (data[i + 2] & 0b11000000) == 0b10000000
                && (data[i + 3] & 0b11000000) == 0b10000000
            {
                i += 4;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

fn main() {
    let data = vec![197, 130, 1];
    let ret = valid_utf8(data);
    println!("ret={ret}");
}

#[test]
fn test_valid_utf8() {
    {
        let data = vec![197, 130, 1];
        let ret = valid_utf8(data);
        assert!(ret);
    }
    {
        let data = vec![235, 140, 4];
        let ret = valid_utf8(data);
        assert!(!ret);
    }
}
