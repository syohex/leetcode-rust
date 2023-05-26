fn stone_game_ii(piles: Vec<i32>) -> i32 {
    let len = piles.len();

    fn f(
        is_alice: usize,
        i: usize,
        len: usize,
        m: usize,
        piles: &Vec<i32>,
        cache: &mut Vec<Vec<Vec<i32>>>,
    ) -> i32 {
        if i >= len {
            return 0;
        }

        if cache[is_alice][i][m] != -1 {
            return cache[is_alice][i][m];
        }

        let limit = std::cmp::min(2 * m, len - i);

        let mut ret = if is_alice == 1 { -1 } else { 1_000_000 };
        let mut score = 0;
        for j in 1..=limit {
            score += piles[i + j - 1];

            let next_m = std::cmp::max(m, j);
            if is_alice == 1 {
                ret = std::cmp::max(ret, score + f(0, i + j, len, next_m, piles, cache));
            } else {
                ret = std::cmp::min(ret, f(1, i + j, len, next_m, piles, cache));
            }
        }

        cache[is_alice][i][m] = ret;
        ret
    }

    let mut cache = vec![vec![vec![-1; len + 1]; len + 1]; 2];
    f(1, 0, len, 1, &piles, &mut cache)
}

fn main() {
    let piles = vec![2, 7, 9, 4, 4];
    let ret = stone_game_ii(piles);
    println!("ret={ret}");
}

#[test]
fn test_stone_game_ii() {
    {
        let piles = vec![2, 7, 9, 4, 4];
        let ret = stone_game_ii(piles);
        assert_eq!(ret, 10);
    }
    {
        let piles = vec![1, 2, 3, 4, 5, 100];
        let ret = stone_game_ii(piles);
        assert_eq!(ret, 104);
    }
    {
        let piles = vec![1];
        let ret = stone_game_ii(piles);
        assert_eq!(ret, 1);
    }
}
