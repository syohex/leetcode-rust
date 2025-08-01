fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut ret = vec![vec![1]];

    for i in 1..num_rows {
        let i = i as usize;
        let mut v = vec![];
        for j in 0..=i {
            if j == 0 || j == i {
                v.push(1);
            } else {
                v.push(ret[i - 1][j - 1] + ret[i - 1][j]);
            }
        }
        ret.push(v);
    }

    ret
}

fn main() {
    let ret = generate(6);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let ret = generate(5);
        let expected = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];
        assert_eq!(ret, expected);
    }
    {
        let ret = generate(1);
        let expected = vec![vec![1]];
        assert_eq!(ret, expected);
    }
}
