fn push_dominoes(dominoes: String) -> String {
    let len = dominoes.len();
    let mut forces = vec![0; len];

    let mut v = 0;
    for (i, c) in dominoes.chars().enumerate() {
        match c {
            'L' => v = 0,
            'R' => v = len as i32,
            _ => {
                if v > 0 {
                    v -= 1;
                }
            }
        }

        forces[i] = v;
    }

    v = 0;
    for (i, c) in dominoes.chars().rev().enumerate() {
        match c {
            'L' => v = len as i32,
            'R' => v = 0,
            _ => {
                if v > 0 {
                    v -= 1;
                }
            }
        }

        forces[len - 1 - i] -= v;
    }

    let mut ret = String::new();
    for f in forces {
        if f > 0 {
            ret.push('R');
        } else if f == 0 {
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
fn test() {
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
