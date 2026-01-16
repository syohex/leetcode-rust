fn maximize_square_area(m: i32, n: i32, h_fences: Vec<i32>, v_fences: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    fn collect_length(len: i32, fences: &[i32]) -> HashSet<i32> {
        let mut v = vec![1];
        for n in fences {
            v.push(*n);
        }
        v.push(len);
        v.sort_unstable();

        let mut s = HashSet::new();
        for i in 0..v.len() {
            for j in (i + 1)..v.len() {
                s.insert(v[j] - v[i]);
            }
        }

        s
    }

    let s1 = collect_length(m, &h_fences);
    let s2 = collect_length(n, &v_fences);

    let mut max_len = -1;
    for len1 in s1 {
        if s2.contains(&len1) {
            max_len = std::cmp::max(max_len, len1);
        }
    }

    if max_len == -1 {
        -1
    } else {
        let len = max_len as i64;
        ((len * len) % 1_000_000_007) as i32
    }
}

fn main() {
    let h_fences = vec![2, 3];
    let v_fences = vec![2];
    let ret = maximize_square_area(4, 3, h_fences, v_fences);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let h_fences = vec![2, 3];
        let v_fences = vec![2];
        let ret = maximize_square_area(4, 3, h_fences, v_fences);
        assert_eq!(ret, 4);
    }
    {
        let h_fences = vec![2];
        let v_fences = vec![4];
        let ret = maximize_square_area(6, 7, h_fences, v_fences);
        assert_eq!(ret, -1);
    }
}
