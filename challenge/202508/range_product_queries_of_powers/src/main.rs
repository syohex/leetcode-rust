fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let modulo = 1_000_000_007i64;
    let mut v = vec![];
    let mut n = n;
    let mut tmp = 1;
    while n > 0 {
        if n % 2 == 1 {
            v.push(tmp);
        }
        tmp *= 2;
        n /= 2;
    }

    let len = v.len();
    let mut muls = vec![vec![0;len];len];
    for i in 0..len {
        let mut tmp = 1i64;
        for j in i..len {
            tmp = (tmp * v[j] as i64) % modulo;
            muls[i][j] = tmp;
        }
    }

    let mut ret = vec![];
    for q in queries {
        ret.push(muls[q[0] as usize][q[1] as usize] as i32);
    }
    ret
}

fn main() {
    let queries = vec![vec![0, 1], vec![2, 2], vec![0, 3]];
    let ret = product_queries(15, queries);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let queries = vec![vec![0, 1], vec![2, 2], vec![0, 3]];
        let expected = vec![2, 4, 64];
        let ret = product_queries(15, queries);
        assert_eq!(ret, expected);
    }
    {
        let queries = vec![vec![0, 0]];
        let expected = vec![2];
        let ret = product_queries(2, queries);
        assert_eq!(ret, expected);
    }
}
