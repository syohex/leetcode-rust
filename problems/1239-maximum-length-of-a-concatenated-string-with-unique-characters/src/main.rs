fn inner(bss: &Vec<Vec<u8>>, pos: usize, table: &mut Vec<bool>) -> i32 {
    let mut ret = table.iter().filter(|&n| *n).count() as i32;
    if pos >= bss.len() {
        return ret;
    }

    for (i, bs) in bss.iter().enumerate().skip(pos) {
        let mut ok = true;
        let mut clear_num = bs.len();
        for (i, &b) in bs.iter().enumerate() {
            let index = (b - b'a') as usize;
            if table[index] {
                ok = false;
                clear_num = i;
                break;
            }

            table[index] = true;
        }

        if ok {
            let r = inner(bss, i + 1, table);
            ret = std::cmp::max(ret, r);
        }

        for &b in bs.iter().take(clear_num) {
            let index = (b - b'a') as usize;
            table[index] = false;
        }
    }

    ret
}
fn max_length(arr: Vec<String>) -> i32 {
    let bss: Vec<Vec<u8>> = arr.iter().map(|s| s.bytes().collect()).collect();
    let mut table = vec![false; 26];
    inner(&bss, 0, &mut table)
}

fn main() {
    let input = vec!["un".to_string(), "iq".to_string(), "ue".to_string()];
    println!("ret={}", max_length(input));
}

#[test]
fn test_max_length() {
    {
        let input = vec!["un".to_string(), "iq".to_string(), "ue".to_string()];
        assert_eq!(max_length(input), 4);
    }
    {
        let input = vec![
            "cha".to_string(),
            "r".to_string(),
            "act".to_string(),
            "ers".to_string(),
        ];
        assert_eq!(max_length(input), 6);
    }
    {
        let input = vec!["abcdefghijklmnopqrstuvwxyz".to_string()];
        assert_eq!(max_length(input), 26);
    }
}
