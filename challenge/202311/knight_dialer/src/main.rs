fn knight_dialer(n: i32) -> i32 {
    use std::collections::HashMap;
    let moves = vec![
        vec![4, 6],
        vec![6, 8],
        vec![7, 9],
        vec![4, 8],
        vec![3, 9, 0],
        vec![],
        vec![1, 7, 0],
        vec![2, 6],
        vec![1, 3],
        vec![2, 4],
    ];

    fn f(
        pos: usize,
        remains: i32,
        moves: &Vec<Vec<usize>>,
        cache: &mut HashMap<(usize, i32), i64>,
    ) -> i64 {
        if remains == 1 {
            return 1;
        }

        if let Some(v) = cache.get(&(pos, remains)) {
            return *v;
        }

        let modulo = 1_000_000_007;
        let mut ret = 0;
        for next in &moves[pos] {
            ret = (ret + f(*next, remains - 1, moves, cache)) % modulo;
        }

        cache.insert((pos, remains), ret);
        ret
    }

    let modulo = 1_000_000_007;
    let mut ret = 0;
    let mut cache = HashMap::new();
    for i in 0..=9 {
        ret = (ret + f(i, n, &moves, &mut cache)) % modulo;
    }

    ret as i32
}

fn main() {
    let ret = knight_dialer(3131);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(knight_dialer(1), 10);
    assert_eq!(knight_dialer(2), 20);
    assert_eq!(knight_dialer(3131), 136006598);
}
