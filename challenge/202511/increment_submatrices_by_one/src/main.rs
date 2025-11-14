fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut prefix = vec![vec![0; n + 1]; n + 1];

    for q in queries {
        let (r1, c1) = (q[0] as usize, q[1] as usize);
        let (r2, c2) = (q[2] as usize, q[3] as usize);

        prefix[r1][c1] += 1;
        prefix[r1][c2 + 1] -= 1;
        prefix[r2 + 1][c1] -= 1;
        prefix[r2 + 1][c2 + 1] += 1;
    }

    let mut ret = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            let v1 = if i >= 1 && j >= 1 {
                ret[i - 1][j - 1]
            } else {
                0
            };
            let v2 = if i >= 1 { ret[i - 1][j] } else { 0 };
            let v3 = if j >= 1 { ret[i][j - 1] } else { 0 };

            ret[i][j] = v2 + v3 - v1 + prefix[i][j];
        }
    }

    ret
}

fn main() {
    let n = 3;
    let queries = vec![vec![1, 1, 2, 2], vec![0, 0, 1, 1]];
    let ret = range_add_queries(n, queries);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let n = 3;
        let queries = vec![vec![1, 1, 2, 2], vec![0, 0, 1, 1]];
        let expected = vec![vec![1, 1, 0], vec![1, 2, 1], vec![0, 1, 1]];
        let ret = range_add_queries(n, queries);
        assert_eq!(ret, expected);
    }
    {
        let n = 2;
        let queries = vec![vec![0, 0, 1, 1]];
        let expected = vec![vec![1, 1], vec![1, 1]];
        let ret = range_add_queries(n, queries);
        assert_eq!(ret, expected);
    }
}
