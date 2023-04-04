fn partition_string(s: String) -> i32 {
    let bs: Vec<u8> = s.bytes().collect();
    let mut window = vec![0; 26];
    let mut ret = 1;
    for b in bs {
        let pos = (b - b'a') as usize;
        if window[pos] != 0 {
            ret += 1;
            window = vec![0; 26];
        }

        window[pos] += 1;
    }

    ret
}

fn main() {
    let s = "abacaba".to_string();
    let ret = partition_string(s);
    println!("ret={ret}");
}

#[test]
fn test_partiton_string() {
    {
        let s = "abacaba".to_string();
        let ret = partition_string(s);
        assert_eq!(ret, 4);
    }
    {
        let s = "ssssss".to_string();
        let ret = partition_string(s);
        assert_eq!(ret, 6);
    }
}
