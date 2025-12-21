fn min_deletion_size(strs: Vec<String>) -> i32 {
    fn is_sorted(v: &[Vec<u8>]) -> bool {
        for i in 1..v.len() {
            if v[i - 1].cmp(&v[i]) == std::cmp::Ordering::Greater {
                return false;
            }
        }

        true
    }

    let strs: Vec<Vec<_>> = strs.into_iter().map(|s| s.bytes().collect()).collect();
    let (rows, cols) = (strs.len(), strs[0].len());

    let mut ret = 0;
    let mut v: Vec<Vec<u8>> = vec![vec![]; rows];
    for col in 0..cols {
        let mut tmp = v.clone();
        for row in 0..rows {
            tmp[row].push(strs[row][col]);
        }

        if is_sorted(&tmp) {
            v = tmp;
        } else {
            ret += 1;
        }
    }

    ret
}

fn main() {
    let strs = vec!["zyx".to_string(), "wvu".to_string(), "tsr".to_string()];
    let ret = min_deletion_size(strs);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let strs = vec!["ca".to_string(), "bb".to_string(), "ac".to_string()];
        let ret = min_deletion_size(strs);
        assert_eq!(ret, 1);
    }
    {
        let strs = vec!["xc".to_string(), "yb".to_string(), "za".to_string()];
        let ret = min_deletion_size(strs);
        assert_eq!(ret, 0);
    }
    {
        let strs = vec!["zyx".to_string(), "wvu".to_string(), "tsr".to_string()];
        let ret = min_deletion_size(strs);
        assert_eq!(ret, 3);
    }
}
