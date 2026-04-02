fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;

    fn f(
        row: usize,
        col: usize,
        specials: i32,
        coins: &Vec<Vec<i32>>,
        cache: &mut HashMap<(usize, usize, i32), i32>,
    ) -> i32 {
        let (rows, cols) = (coins.len(), coins[0].len());
        if row >= rows || col >= cols {
            return i32::MIN;
        }

        let v = coins[row][col];
        if row == rows - 1 && col == cols - 1 {
            if specials > 0 {
                return std::cmp::max(0, v);
            }

            return v;
        }

        let key = (row, col, specials);
        if let Some(v) = cache.get(&key) {
            return *v;
        }

        let mut ret = std::cmp::max(
            f(row + 1, col, specials, coins, cache),
            f(row, col + 1, specials, coins, cache),
        ) + v;

        if specials > 0 && v < 0 {
            ret = std::cmp::max(
                ret,
                std::cmp::max(
                    f(row + 1, col, specials - 1, coins, cache),
                    f(row, col + 1, specials - 1, coins, cache),
                ),
            );
        }

        cache.insert(key, ret);
        ret
    }

    let mut cache = HashMap::new();
    f(0, 0, 2, &coins, &mut cache)
}

fn main() {
    let coins = vec![vec![0, 1, -1], vec![1, -2, 3], vec![2, -3, 4]];
    let ret = maximum_amount(coins);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let coins = vec![
            vec![-7, 12, 12, 13],
            vec![-6, 19, 19, -6],
            vec![9, -2, -10, 16],
            vec![-4, 14, -10, -9],
        ];
        let ret = maximum_amount(coins);
        assert_eq!(ret, 60);
    }
    {
        let coins = vec![vec![0, 1, -1], vec![1, -2, 3], vec![2, -3, 4]];
        let ret = maximum_amount(coins);
        assert_eq!(ret, 8);
    }
    {
        let coins = vec![vec![10, 10, 10], vec![10, 10, 10]];
        let ret = maximum_amount(coins);
        assert_eq!(ret, 40);
    }
}
