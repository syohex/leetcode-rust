fn push_dominoes(dominoes: String) -> String {
    let len = dominoes.len();
    let mut vals = vec![0i32; len];

    let mut value = 0i32;
    for (i, c) in dominoes.chars().enumerate() {
        match c {
            'L' => value = 0,
            'R' => value = len as i32,
            _ => {
                if value != 0 {
                    value -= 1;
                }
            }
        }

        vals[i] = value;
    }

    value = 0;
    for (i, c) in dominoes.chars().rev().enumerate() {
        match c {
            'L' => value = len as i32,
            'R' => value = 0,
            _ => {
                if value != 0 {
                    value -= 1;
                }
            }
        }

        vals[len - 1 - i] -= value;
    }

    let mut ret = String::new();
    for val in vals {
        if val > 0 {
            ret.push('R');
        } else if val == 0 {
            ret.push('.');
        } else {
            ret.push('L');
        }
    }

    ret
}

fn main() {
    let dominoes = ".L.R...LR..L..".to_string();
    let ret = push_dominoes(dominoes);
    println!("ret={ret}");
}

#[test]
fn test_push_dominoes() {
    {
        let dominoes = "RR.L".to_string();
        let ret = push_dominoes(dominoes);
        assert_eq!(ret, "RR.L");
    }
    {
        let dominoes = ".L.R...LR..L..".to_string();
        let ret = push_dominoes(dominoes);
        assert_eq!(ret, "LL.RR.LLRRLL..");
    }
}
