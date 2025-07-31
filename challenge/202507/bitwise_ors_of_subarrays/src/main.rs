fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    let mut s = HashSet::new();
    let mut prev = HashSet::new();
    prev.insert(0);

    for n in arr {
        let mut tmp = HashSet::new();
        tmp.insert(n);

        for &m in &prev {
            tmp.insert(n | m);
        }

        for &m in &tmp {
            s.insert(m);
        }

        prev = tmp;
    }

    s.len() as i32
}

fn main() {
    let arr = vec![1, 2, 4];
    let ret = subarray_bitwise_o_rs(arr);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let arr = vec![0];
        let ret = subarray_bitwise_o_rs(arr);
        assert_eq!(ret, 1);
    }
    {
        let arr = vec![1, 1, 2];
        let ret = subarray_bitwise_o_rs(arr);
        assert_eq!(ret, 3);
    }
    {
        let arr = vec![1, 2, 4];
        let ret = subarray_bitwise_o_rs(arr);
        assert_eq!(ret, 6);
    }
}
