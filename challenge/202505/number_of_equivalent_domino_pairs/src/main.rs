fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;

    let mut h = HashMap::new();
    for d in dominoes {
        if d[0] < d[1] {
            *h.entry((d[0], d[1])).or_insert(0) += 1;
        } else {
            *h.entry((d[1], d[0])).or_insert(0) += 1;
        }
    }

    let mut ret = 0;
    for v in h.values().filter(|&&v| v >= 2) {
        ret += (v * (v - 1)) / 2;
    }

    ret
}

fn main() {
    let dominoes = vec![vec![1, 2], vec![1, 2], vec![1, 1], vec![1, 2], vec![2, 2]];
    let ret = num_equiv_domino_pairs(dominoes);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let dominoes = vec![vec![1, 2], vec![2, 1], vec![3, 4], vec![5, 6]];
        let ret = num_equiv_domino_pairs(dominoes);
        assert_eq!(ret, 1);
    }
    {
        let dominoes = vec![vec![1, 2], vec![1, 2], vec![1, 1], vec![1, 2], vec![2, 2]];
        let ret = num_equiv_domino_pairs(dominoes);
        assert_eq!(ret, 3);
    }
}
