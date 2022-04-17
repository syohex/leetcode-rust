fn digit_sum(s: String, k: i32) -> String {
    let mut ret = s;
    let k = k as usize;

    while ret.len() > k {
        let bs = ret.bytes().collect::<Vec<u8>>();
        let mut tmp = String::new();
        for chunk in bs.chunks(k) {
            let n = chunk.iter().fold(0, |acc, b| acc + (b - b'0') as i32);
            tmp.push_str(format!("{n}").as_str());
        }

        ret = tmp;
    }

    ret
}

fn main() {
    let s = "11111222223".to_string();
    let ret = digit_sum(s, 3);
    println!("ret={ret}");
}

#[test]
fn test_digit_sum() {
    {
        let s = "11111222223".to_string();
        let ret = digit_sum(s, 3);
        assert_eq!(ret, "135".to_string());
    }
    {
        let s = "00000000".to_string();
        let ret = digit_sum(s, 3);
        assert_eq!(ret, "000".to_string());
    }
    {
        let s = "71818186138735364590516322993378229838446988388364431324753408563431136824898916288399".to_string();
        let ret = digit_sum(s, 85);
        assert_eq!(ret, "4169".to_string());
    }
}
