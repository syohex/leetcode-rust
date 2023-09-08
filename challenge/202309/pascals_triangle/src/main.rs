fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let num_rows = num_rows as usize;
    let mut ret = vec![vec![1]];

    for i in 1..num_rows {
        let mut v = vec![];
        for j in 0..=i {
            if j == 0 {
                v.push(ret[i - 1][0]);
            } else if j == i {
                v.push(ret[i - 1][i - 1]);
            } else {
                v.push(ret[i - 1][j - 1] + ret[i - 1][j]);
            }
        }

        ret.push(v);
    }

    ret
}

fn main() {
    let ret = generate(7);
    println!("ret={ret:?}");
}

#[test]
fn test_generate() {
    {
        let expected = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];
        let ret = generate(5);
        assert_eq!(ret, expected);
    }
    {
        let expected = vec![vec![1]];
        let ret = generate(1);
        assert_eq!(ret, expected);
    }
}
