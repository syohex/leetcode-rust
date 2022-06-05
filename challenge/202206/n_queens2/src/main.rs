fn total_n_queens(n: i32) -> i32 {
    fn f(p: usize, n: usize, acc: &mut Vec<usize>) -> i32 {
        if p == n {
            return 1;
        }

        let mut ret = 0;
        for i in 0..n {
            if acc.iter().take(p).enumerate().all(|(j, &m)| {
                let diff = if m <= i { i - m } else { m - i };
                i != m && diff != p - j
            }) {
                acc[p] = i;
                ret += f(p + 1, n, acc);
            }
        }

        ret
    }

    let n = n as usize;
    let mut acc = vec![0; n];
    f(0, n, &mut acc)
}

fn main() {
    let ret = total_n_queens(8);
    println!("ret={ret}");
}

#[test]
fn test_total_n_queens() {
    assert_eq!(total_n_queens(4), 2);
    assert_eq!(total_n_queens(1), 1);
    assert_eq!(total_n_queens(8), 92);
}
