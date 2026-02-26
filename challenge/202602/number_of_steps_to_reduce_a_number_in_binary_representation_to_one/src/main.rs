fn num_steps(s: String) -> i32 {
    let mut cs: Vec<_> = s.chars().collect();
    let mut ret = 0;

    loop {
        let len = cs.len();
        if len == 1 && cs[0] == '1' {
            return ret;
        }

        ret += 1;
        if cs[len - 1] == '0' {
            cs.pop();
        } else {
            let mut zero_found = false;
            for i in (0..len).rev() {
                if cs[i] == '0' {
                    zero_found = true;
                    cs[i] = '1';
                    break;
                }
                cs[i] = '0';
            }

            if !zero_found {
                let mut tmp = vec!['1'];
                tmp.append(&mut cs);
                cs = tmp;
            }
        }
    }
}

fn main() {
    let ret = num_steps("1101".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(num_steps("1101".to_string()), 6);
    assert_eq!(num_steps("10".to_string()), 1);
    assert_eq!(num_steps("1".to_string()), 0);
}
