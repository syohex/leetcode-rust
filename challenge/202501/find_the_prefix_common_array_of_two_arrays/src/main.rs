fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    use std::collections::HashSet;

    let mut sa = HashSet::new();
    let mut sb = HashSet::new();

    let mut ret = vec![];
    for (m, n) in a.into_iter().zip(b.into_iter()) {
        sa.insert(m);
        sb.insert(n);

        ret.push(sa.intersection(&sb).count() as i32)
    }

    ret
}

fn main() {
    let a = vec![1, 3, 2, 4];
    let b = vec![3, 1, 2, 4];
    let ret = find_the_prefix_common_array(a, b);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let a = vec![1, 3, 2, 4];
        let b = vec![3, 1, 2, 4];
        let expected = vec![0, 2, 3, 4];
        let ret = find_the_prefix_common_array(a, b);
        assert_eq!(ret, expected);
    }
    {
        let a = vec![2, 3, 1];
        let b = vec![3, 1, 2];
        let expected = vec![0, 1, 3];
        let ret = find_the_prefix_common_array(a, b);
        assert_eq!(ret, expected);
    }
}
