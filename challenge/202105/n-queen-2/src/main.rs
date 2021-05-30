fn nqueen(n: i32, p: i32, v: &mut Vec<i32>, ret: &mut i32) {
    if p == n {
        *ret += 1;
        return;
    }

    for i in 0..n {
        if v.iter()
            .enumerate()
            .take(p as usize)
            .all(|(j, m)| i != *m && (i - *m).abs() != p - j as i32)
        {
            v[p as usize] = i;
            nqueen(n, p + 1, v, ret);
        }
    }
}

fn total_n_queens(n: i32) -> i32 {
    let mut v = vec![0; n as usize];
    let mut ret = 0;

    nqueen(n, 0, &mut v, &mut ret);
    ret
}

fn main() {
    let ret = total_n_queens(10);
    println!("ret={}", ret);
}

#[test]
fn test_total_n_queens() {
    assert_eq!(total_n_queens(4), 2);
    assert_eq!(total_n_queens(1), 1);
    assert_eq!(total_n_queens(10), 724);
}
