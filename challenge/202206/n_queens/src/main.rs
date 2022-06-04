fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    fn q(pos: usize, n: usize) -> String {
        let mut s = String::new();

        for _ in 0..pos {
            s.push('.');
        }
        s.push('Q');

        for _ in (pos + 1)..n {
            s.push('.');
        }

        s
    }

    fn f(pos: usize, n: usize, acc: &mut Vec<usize>, ret: &mut Vec<Vec<String>>) {
        if pos >= n {
            let v = acc.iter().map(|p| q(*p, n)).collect();
            ret.push(v);
            return;
        }

        for i in 0..n {
            let mut ok = true;
            for j in 0..pos {
                let diff = if acc[j] < i { i - acc[j] } else { acc[j] - i };
                if acc[j] == i || diff == pos - j {
                    ok = false;
                    break;
                }
            }

            if ok {
                acc[pos] = i;
                f(pos + 1, n, acc, ret);
            }
        }
    }

    let n = n as usize;
    let mut acc = vec![0; n];
    let mut ret = vec![];

    f(0, n, &mut acc, &mut ret);

    ret
}

fn main() {
    let ret = solve_n_queens(8);
    println!("ret={:?}", ret);
}

#[test]
fn test_solve_n_queens() {
    fn check(got: Vec<Vec<String>>, expected: Vec<Vec<String>>) {
        use std::collections::HashSet;

        let g: HashSet<Vec<String>> = got.into_iter().collect();
        let e: HashSet<Vec<String>> = expected.into_iter().collect();

        assert_eq!(g, e);
    }

    {
        let ret = solve_n_queens(4);
        let expected = vec![
            vec![
                ".Q..".to_string(),
                "...Q".to_string(),
                "Q...".to_string(),
                "..Q.".to_string(),
            ],
            vec![
                "..Q.".to_string(),
                "Q...".to_string(),
                "...Q".to_string(),
                ".Q..".to_string(),
            ],
        ];
        check(ret, expected);
    }
}
