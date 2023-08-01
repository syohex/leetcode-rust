fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    fn f(start: i32, n: i32, k: i32, acc: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
        if acc.len() == k as usize {
            ret.push(acc.clone());
            return;
        }

        for i in start..=n {
            acc.push(i);
            f(i + 1, n, k, acc, ret);
            acc.pop();
        }
    }

    let mut acc = vec![];
    let mut ret = vec![];

    f(1, n, k, &mut acc, &mut ret);
    ret
}

fn main() {
    let ret = combine(4, 3);
    println!("ret={ret:?}");
}

#[test]
fn test_combine() {
    fn check(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) {
        use std::collections::HashSet;

        let s1: HashSet<Vec<i32>> = a.into_iter().collect();
        let s2: HashSet<Vec<i32>> = b.into_iter().collect();
        assert_eq!(s1, s2);
    }

    {
        let expected = vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 3],
            vec![2, 4],
            vec![3, 4],
        ];
        let ret = combine(4, 2);
        check(ret, expected);
    }
    {
        let expected = vec![vec![1]];
        let ret = combine(1, 1);
        check(ret, expected);
    }
}
