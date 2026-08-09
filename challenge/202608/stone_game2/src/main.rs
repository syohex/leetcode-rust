fn stone_game_ii(piles: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    fn f(
        i: usize,
        m: usize,
        is_alice: bool,
        piles: &[i32],
        cache: &mut HashMap<(bool, usize, usize), i32>,
    ) -> i32 {
        let len = piles.len();
        if i >= len {
            return 0;
        }

        let key = (is_alice, i, m);
        if let Some(v) = cache.get(&key) {
            return *v;
        }

        let limit = std::cmp::min(2 * m, len - i);
        let mut stones = 0;
        let mut ret = if is_alice { 0 } else { 1_000_000 };
        for j in 1..=limit {
            stones += piles[i + j - 1];

            let next_m = std::cmp::max(m, j);
            if is_alice {
                ret = std::cmp::max(ret, stones + f(i + j, next_m, false, piles, cache));
            } else {
                ret = std::cmp::min(ret, f(i + j, next_m, true, piles, cache));
            }
        }

        cache.insert(key, ret);
        ret
    }

    let mut cache = HashMap::new();
    f(0, 1, true, &piles, &mut cache)
}

fn main() {
    let ret = stone_game_ii(vec![2, 7, 9, 4, 4]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(stone_game_ii(vec![2, 7, 9, 4, 4]), 10);
    assert_eq!(stone_game_ii(vec![1, 2, 3, 4, 5, 100]), 104);
}
