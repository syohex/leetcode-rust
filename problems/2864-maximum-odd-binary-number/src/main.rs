fn maximum_odd_binary_number(s: String) -> String {
    let ones = s.chars().filter(|&c| c == '1').count();
    let zeros = s.len() - ones;

    let mut ret = String::new();
    for _ in 0..(ones - 1) {
        ret.push('1');
    }

    for _ in 0..zeros {
        ret.push('0');
    }

    ret.push('1');

    ret
}

fn main() {
    let s = "0101".to_string();
    let ret = maximum_odd_binary_number(s);
    println!("ret={ret}");
}

#[test]
fn test_maximum_odd_binary_number() {
    {
        let s = "010".to_string();
        let ret = maximum_odd_binary_number(s);
        assert_eq!(ret, "001");
    }
    {
        let s = "0101".to_string();
        let ret = maximum_odd_binary_number(s);
        assert_eq!(ret, "1001");
    }
    {
        let s = "1111".to_string();
        let ret = maximum_odd_binary_number(s);
        assert_eq!(ret, "1111");
    }
    {
        let s = "1001".to_string();
        let ret = maximum_odd_binary_number(s);
        assert_eq!(ret, "1001");
    }
    {
        let s = "011".to_string();
        let ret = maximum_odd_binary_number(s);
        assert_eq!(ret, "101");
    }
    {
        let s = "00111".to_string();
        let ret = maximum_odd_binary_number(s);
        assert_eq!(ret, "11001");
    }
    {
        let s = "1".to_string();
        let ret = maximum_odd_binary_number(s);
        assert_eq!(ret, "1");
    }
}
