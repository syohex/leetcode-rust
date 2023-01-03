fn min_deletion_size(strs: Vec<String>) -> i32 {
    let strs: Vec<Vec<u8>> = strs.into_iter().map(|s| s.bytes().collect()).collect();
    let mut ret = 0;
    for i in 0..strs[0].len() {
        let mut sorted = true;
        for j in 1..strs.len() {
            if strs[j - 1][i] > strs[j][i] {
                sorted = false;
                break;
            }
        }

        if !sorted {
            ret += 1;
        }
    }

    ret
}

fn main() {
    let strs = vec!["cba".to_string(), "daf".to_string(), "ghi".to_string()];
    let ret = min_deletion_size(strs);
    println!("ret={ret}");
}

#[test]
fn test_min_deletion_size() {
    {
        let strs = vec!["cba".to_string(), "daf".to_string(), "ghi".to_string()];
        let ret = min_deletion_size(strs);
        assert_eq!(ret, 1);
    }
    {
        let strs = vec!["a".to_string(), "b".to_string()];
        let ret = min_deletion_size(strs);
        assert_eq!(ret, 0);
    }
    {
        let strs = vec!["zyx".to_string(), "wvu".to_string(), "tsr".to_string()];
        let ret = min_deletion_size(strs);
        assert_eq!(ret, 3);
    }
    {
        let strs = vec!["rrjk".to_string(), "furt".to_string(), "guzm".to_string()];
        let ret = min_deletion_size(strs);
        assert_eq!(ret, 2);
    }
}
