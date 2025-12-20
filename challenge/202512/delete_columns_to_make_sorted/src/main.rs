fn min_deletion_size(strs: Vec<String>) -> i32 {
    let mut ret = 0;
    let vs: Vec<Vec<_>> = strs.into_iter().map(|s| s.bytes().collect()).collect();

    let (rows, cols) = (vs.len(), vs[0].len());
    for j in 0..cols {
        for i in 1..rows {
            if vs[i - 1][j] > vs[i][j] {
                ret += 1;
                break;
            }
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
fn test() {
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
}
